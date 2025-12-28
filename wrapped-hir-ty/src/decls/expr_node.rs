macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! expr_node {
    () => {
        deps!();
        fn expr_node (body_source_map : & BodySourceMap , expr : ExprId , db : & TestDB ,) -> Option < InFile < SyntaxNode > > { Some (match body_source_map . expr_syntax (expr) { Ok (sp) => { let root = db . parse_or_expand (sp . file_id) ; sp . map (| ptr | ptr . to_node (& root) . syntax () . clone ()) } Err (SyntheticSyntax) => return None , }) }
    };
}

expr_node!()