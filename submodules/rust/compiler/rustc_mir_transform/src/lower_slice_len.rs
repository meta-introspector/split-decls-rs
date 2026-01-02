mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkitem!{mkstruct!{pub (super) struct LowerSliceLenCalls ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for LowerSliceLenCalls { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () > 0 } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let language_items = tcx . lang_items () ; let Some (slice_len_fn_item_def_id) = language_items . slice_len_fn () else { return ; } ; let basic_blocks = body . basic_blocks . as_mut_preserves_cfg () ; for block in basic_blocks { lower_slice_len_call (block , slice_len_fn_item_def_id) ; } } fn is_required (& self) -> bool { false } }}}

macro_rules! lower_slice_len_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lower_slice_len_call in module {}", module_path!());
    };
}

mkfn!{
    lower_slice_len_call_introspect!();
    fn lower_slice_len_call < 'tcx > (block : & mut BasicBlockData < 'tcx > , slice_len_fn_item_def_id : DefId) { let terminator = block . terminator () ; if let TerminatorKind :: Call { func , args , destination , target : Some (bb) , call_source : CallSource :: Normal , .. } = & terminator . kind && let [arg] = & args [..] && let Some ((fn_def_id , _)) = func . const_fn_def () && fn_def_id == slice_len_fn_item_def_id { let r_value = Rvalue :: UnaryOp (UnOp :: PtrMetadata , arg . node . clone ()) ; let len_statement_kind = StatementKind :: Assign (Box :: new ((* destination , r_value))) ; let add_statement = Statement :: new (terminator . source_info , len_statement_kind) ; let new_terminator_kind = TerminatorKind :: Goto { target : * bb } ; block . statements . push (add_statement) ; block . terminator_mut () . kind = new_terminator_kind ; } }
}