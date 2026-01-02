mkuse!{use rustc_abi :: Size ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_middle :: mir :: visit :: Visitor as MirVisitor ;}
mkuse!{use rustc_middle :: mir :: { self , Location , traversal } ;}
mkuse!{use rustc_middle :: ty :: { self , AssocTag , Instance , Ty , TyCtxt , TypeFoldable } ;}
mkuse!{use rustc_session :: lint :: builtin :: LARGE_ASSIGNMENTS ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_span :: { Ident , Span , sym } ;}
mkuse!{use tracing :: { debug , trace } ;}
mkuse!{use crate :: errors :: LargeAssignmentsLint ;}
mkitem!{mkstruct!{struct MoveCheckVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > , # [doc = " Spans for move size lints already emitted. Helps avoid duplicate lints."] move_size_spans : Vec < Span > , }}}

macro_rules! check_moves_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_moves in module {}", module_path!());
    };
}

mkfn!{
    check_moves_introspect!();
    pub (crate) fn check_moves < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { let mut visitor = MoveCheckVisitor { tcx , instance , body , move_size_spans : vec ! [] } ; for (bb , data) in traversal :: mono_reachable (body , tcx , instance) { visitor . visit_basic_block_data (bb , data) } }
}
mkitem!{mkimpl!{impl < 'tcx > MirVisitor < 'tcx > for MoveCheckVisitor < 'tcx > { fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { match terminator . kind { mir :: TerminatorKind :: Call { ref func , ref args , ref fn_span , .. } | mir :: TerminatorKind :: TailCall { ref func , ref args , ref fn_span } => { let callee_ty = func . ty (self . body , self . tcx) ; let callee_ty = self . monomorphize (callee_ty) ; self . check_fn_args_move_size (callee_ty , args , * fn_span , location) ; } _ => { } } } fn visit_operand (& mut self , operand : & mir :: Operand < 'tcx > , location : Location) { self . check_operand_move_size (operand , location) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > MoveCheckVisitor < 'tcx > { fn monomorphize < T > (& self , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { trace ! ("monomorphize: self.instance={:?}" , self . instance) ; self . instance . instantiate_mir_and_normalize_erasing_regions (self . tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (value) ,) } fn check_operand_move_size (& mut self , operand : & mir :: Operand < 'tcx > , location : Location) { let limit = self . tcx . move_size_limit () ; if limit . 0 == 0 { return ; } let source_info = self . body . source_info (location) ; debug ! (? source_info) ; if let Some (too_large_size) = self . operand_size_if_too_large (limit , operand) { self . lint_large_assignment (limit . 0 , too_large_size , location , source_info . span) ; } ; } pub (super) fn check_fn_args_move_size (& mut self , callee_ty : Ty < 'tcx > , args : & [Spanned < mir :: Operand < 'tcx > >] , fn_span : Span , location : Location ,) { let limit = self . tcx . move_size_limit () ; if limit . 0 == 0 { return ; } if args . is_empty () { return ; } let ty :: FnDef (def_id , _) = * callee_ty . kind () else { return ; } ; if self . tcx . skip_move_check_fns (()) . contains (& def_id) { return ; } debug ! (? def_id , ? fn_span) ; for arg in args { let operand : & mir :: Operand < 'tcx > = & arg . node ; if let mir :: Operand :: Move (_) = operand { continue ; } if let Some (too_large_size) = self . operand_size_if_too_large (limit , operand) { self . lint_large_assignment (limit . 0 , too_large_size , location , arg . span) ; } ; } } fn operand_size_if_too_large (& mut self , limit : Limit , operand : & mir :: Operand < 'tcx > ,) -> Option < Size > { let ty = operand . ty (self . body , self . tcx) ; let ty = self . monomorphize (ty) ; let Ok (layout) = self . tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (ty)) else { return None ; } ; if layout . size . bytes_usize () > limit . 0 { debug ! (? layout) ; Some (layout . size) } else { None } } fn lint_large_assignment (& mut self , limit : usize , too_large_size : Size , location : Location , span : Span ,) { let source_info = self . body . source_info (location) ; let lint_root = source_info . scope . lint_root (& self . body . source_scopes) ; let Some (lint_root) = lint_root else { return ; } ; let reported_span = self . body . source_scopes . get (source_info . scope) . and_then (| source_scope_data | source_scope_data . inlined) . map (| (_ , call_site) | call_site) . unwrap_or (span) ; for previously_reported_span in & self . move_size_spans { if previously_reported_span . overlaps (reported_span) { return ; } } self . tcx . emit_node_span_lint (LARGE_ASSIGNMENTS , lint_root , reported_span , LargeAssignmentsLint { span : reported_span , size : too_large_size . bytes () , limit : limit as u64 , } ,) ; self . move_size_spans . push (reported_span) ; } }}}

macro_rules! assoc_fn_of_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assoc_fn_of_type in module {}", module_path!());
    };
}

mkfn!{
    assoc_fn_of_type_introspect!();
    fn assoc_fn_of_type < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , fn_ident : Ident) -> Option < DefId > { for impl_def_id in tcx . inherent_impls (def_id) { if let Some (new) = tcx . associated_items (impl_def_id) . find_by_ident_and_kind (tcx , fn_ident , AssocTag :: Fn , def_id ,) { return Some (new . def_id) ; } } None }
}

macro_rules! skip_move_check_fns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function skip_move_check_fns in module {}", module_path!());
    };
}

mkfn!{
    skip_move_check_fns_introspect!();
    pub (crate) fn skip_move_check_fns (tcx : TyCtxt < '_ > , _ : ()) -> FxIndexSet < DefId > { let fns = [(tcx . lang_items () . owned_box () , "new") , (tcx . get_diagnostic_item (sym :: Rc) , "new") , (tcx . get_diagnostic_item (sym :: Arc) , "new") ,] ; fns . into_iter () . filter_map (| (def_id , fn_name) | { def_id . and_then (| def_id | assoc_fn_of_type (tcx , def_id , Ident :: from_str (fn_name))) }) . collect () }
}