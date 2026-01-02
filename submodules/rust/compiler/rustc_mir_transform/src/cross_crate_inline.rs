mkuse!{use rustc_hir :: attrs :: InlineAttr ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: mir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: { InliningThreshold , OptLevel } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: { inline , pass_manager as pm } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { providers . cross_crate_inlinable = cross_crate_inlinable ; }
}

macro_rules! cross_crate_inlinable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cross_crate_inlinable in module {}", module_path!());
    };
}

mkfn!{
    cross_crate_inlinable_introspect!();
    fn cross_crate_inlinable (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let codegen_fn_attrs = tcx . codegen_fn_attrs (def_id) ; if codegen_fn_attrs . contains_extern_indicator () { return false ; } match tcx . def_kind (def_id) { DefKind :: Ctor (..) | DefKind :: Closure | DefKind :: SyntheticCoroutineBody => return true , DefKind :: Fn | DefKind :: AssocFn => { } _ => return false , } if tcx . sess . opts . unstable_opts . cross_crate_inline_threshold == InliningThreshold :: Always { return true ; } if tcx . has_attr (def_id , sym :: rustc_intrinsic) { return true ; } match codegen_fn_attrs . inline { InlineAttr :: Never => return false , InlineAttr :: Hint | InlineAttr :: Always | InlineAttr :: Force { .. } => return true , _ => { } } if tcx . sess . opts . unstable_opts . hint_mostly_unused { return true ; } let sig = tcx . fn_sig (def_id) . instantiate_identity () ; for ty in sig . inputs () . skip_binder () . iter () . chain (std :: iter :: once (& sig . output () . skip_binder ())) { if ty == & tcx . types . f16 || ty == & tcx . types . f128 { return true ; } } if tcx . sess . opts . incremental . is_some () { return false ; } let inliner_will_run = pm :: should_run_pass (tcx , & inline :: Inline , pm :: Optimizations :: Allowed) || inline :: ForceInline :: should_run_pass_for_callee (tcx , def_id . to_def_id ()) ; if matches ! (tcx . sess . opts . optimize , OptLevel :: No) && ! inliner_will_run { return false ; } if ! tcx . is_mir_available (def_id) { return false ; } let threshold = match tcx . sess . opts . unstable_opts . cross_crate_inline_threshold { InliningThreshold :: Always => return true , InliningThreshold :: Sometimes (threshold) => threshold , InliningThreshold :: Never => return false , } ; let mir = tcx . optimized_mir (def_id) ; let mut checker = CostChecker { tcx , callee_body : mir , calls : 0 , statements : 0 , landing_pads : 0 , resumes : 0 } ; checker . visit_body (mir) ; checker . calls == 0 && checker . resumes == 0 && checker . landing_pads == 0 && checker . statements <= threshold }
}
mkitem!{mkstruct!{struct CostChecker < 'b , 'tcx > { tcx : TyCtxt < 'tcx > , callee_body : & 'b Body < 'tcx > , calls : usize , statements : usize , landing_pads : usize , resumes : usize , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CostChecker < '_ , 'tcx > { fn visit_statement (& mut self , statement : & Statement < 'tcx > , _ : Location) { match statement . kind { StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: Deinit (_) | StatementKind :: Nop => { } _ => self . statements += 1 , } } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , _ : Location) { let tcx = self . tcx ; match terminator . kind { TerminatorKind :: Drop { ref place , unwind , .. } => { let ty = place . ty (self . callee_body , tcx) . ty ; if ! ty . is_trivially_pure_clone_copy () { self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } } TerminatorKind :: Call { ref func , unwind , .. } => { if let Some ((fn_def_id , _)) = func . const_fn_def () && self . tcx . has_attr (fn_def_id , sym :: rustc_intrinsic) { return ; } self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: Assert { unwind , .. } => { self . calls += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: UnwindResume => self . resumes += 1 , TerminatorKind :: InlineAsm { unwind , .. } => { self . statements += 1 ; if let UnwindAction :: Cleanup (_) = unwind { self . landing_pads += 1 ; } } TerminatorKind :: Return => { } _ => self . statements += 1 , } } }}}