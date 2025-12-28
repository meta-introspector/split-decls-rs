macro_rules! adjust {
    () => {
        fn adjust (db : & dyn HirDatabase , scopes : & ExprScopes , source_map : & BodySourceMap , expr_range : TextRange , from_file : HirFileId , offset : TextSize ,) -> Option < ScopeId > { let child_scopes = scopes . scope_by_expr () . iter () . filter_map (| (id , scope) | { let source = source_map . expr_syntax (id) . ok () ? ; if source . file_id != from_file { return None ; } let root = source . file_syntax (db) ; let node = source . value . to_node (& root) ; Some ((node . syntax () . text_range () , scope)) }) . filter (| & (range , _) | { range . start () <= offset && expr_range . contains_range (range) && range != expr_range }) ; child_scopes . max_by (| & (r1 , _) , & (r2 , _) | { if r1 . contains_range (r2) { std :: cmp :: Ordering :: Greater } else if r2 . contains_range (r1) { std :: cmp :: Ordering :: Less } else { r1 . start () . cmp (& r2 . start ()) } }) . map (| (_ptr , scope) | * scope) }
    };
}

adjust!();