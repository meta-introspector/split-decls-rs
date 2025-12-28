macro_rules! deps {
    () => {
        PathCompletionCtx!();
        Builder!();
        CompletionContext!();
    };
}

macro_rules! path_ref_match {
    () => {
        deps!();
        fn path_ref_match (completion : & CompletionContext < '_ > , path_ctx : & PathCompletionCtx < '_ > , ty : & hir :: Type < '_ > , item : & mut Builder ,) { if let Some (original_path) = & path_ctx . original_path { if let Some (original_path) = completion . sema . original_ast_node (original_path . clone ()) && let Some (ref_mode) = compute_ref_match (completion , ty) { item . ref_match (ref_mode , original_path . syntax () . text_range () . start ()) ; } } else { if let Some (ref_mode) = compute_ref_match (completion , ty) { item . ref_match (ref_mode , completion . position . offset) ; } } }
    };
}

path_ref_match!();