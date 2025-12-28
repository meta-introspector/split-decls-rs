macro_rules! deps {
    () => {
        ItemLowerer!();
    };
}

macro_rules! lower_to_hir {
    () => {
        deps!();
        pub fn lower_to_hir (tcx : TyCtxt < '_ > , () : ()) -> hir :: Crate < '_ > { let sess = tcx . sess ; tcx . ensure_done () . output_filenames (()) ; tcx . ensure_done () . early_lint_checks (()) ; tcx . ensure_done () . debugger_visualizers (LOCAL_CRATE) ; tcx . ensure_done () . get_lang_items (()) ; let (mut resolver , krate) = tcx . resolver_for_lowering () . steal () ; let ast_index = index_crate (& resolver . node_id_to_def_id , & krate) ; let mut owners = IndexVec :: from_fn_n (| _ | hir :: MaybeOwner :: Phantom , tcx . definitions_untracked () . def_index_count () ,) ; let mut lowerer = item :: ItemLowerer { tcx , resolver : & mut resolver , ast_index : & ast_index , owners : & mut owners , } ; for def_id in ast_index . indices () { lowerer . lower_node (def_id) ; } drop (ast_index) ; let prof = sess . prof . clone () ; spawn (move | | { let _timer = prof . verbose_generic_activity ("drop_ast") ; drop (krate) ; }) ; let opt_hir_hash = if tcx . needs_crate_hash () { Some (compute_hir_hash (tcx , & owners)) } else { None } ; hir :: Crate { owners , opt_hir_hash } }
    };
}

lower_to_hir!()