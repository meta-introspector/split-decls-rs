macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! pat_node {
    () => {
        deps!();
        fn pat_node (body_source_map : & BodySourceMap , pat : PatId , db : & TestDB ,) -> Option < InFile < SyntaxNode > > { Some (match body_source_map . pat_syntax (pat) { Ok (sp) => { let root = db . parse_or_expand (sp . file_id) ; sp . map (| ptr | ptr . to_node (& root) . syntax () . clone ()) } Err (SyntheticSyntax) => return None , }) }
    };
}

pat_node!()