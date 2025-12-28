macro_rules! deps {
    () => {
        ExpandDatabase!();
        HirFileId!();
    };
}

macro_rules! ast_id_map {
    () => {
        deps!();
        fn ast_id_map (db : & dyn ExpandDatabase , file_id : HirFileId) -> triomphe :: Arc < AstIdMap > { triomphe :: Arc :: new (AstIdMap :: from_source (& db . parse_or_expand (file_id))) }
    };
}

ast_id_map!();