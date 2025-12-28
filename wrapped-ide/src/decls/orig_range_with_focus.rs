macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! orig_range_with_focus {
    () => {
        deps!();
        # [doc = " Returns the original range of the syntax node, and the range of the name mapped out of macro expansions"] # [doc = " May return two results if the mapped node originates from a macro definition in which case the"] # [doc = " second result is the creating macro call."] fn orig_range_with_focus (db : & RootDatabase , hir_file : HirFileId , value : & SyntaxNode , name : Option < impl AstNode > ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { orig_range_with_focus_r (db , hir_file , value . text_range () , name . map (| it | it . syntax () . text_range ()) ,) }
    };
}

orig_range_with_focus!();