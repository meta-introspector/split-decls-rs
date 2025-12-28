macro_rules! id_to_string {
    () => {
        pub fn id_to_string (cx : & dyn rustc_hir :: intravisit :: HirTyCtxt < '_ > , hir_id : HirId) -> String { to_string (& cx , | s | s . print_node (cx . hir_node (hir_id))) }
    };
}

id_to_string!();