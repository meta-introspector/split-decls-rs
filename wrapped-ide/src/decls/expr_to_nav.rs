macro_rules! deps {
    () => {
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! expr_to_nav {
    () => {
        deps!();
        fn expr_to_nav (db : & RootDatabase , InFile { file_id , value } : InFile < ast :: Expr > , focus_range : Option < TextRange > ,) -> UpmappingResult < NavigationTarget > { let kind = SymbolKind :: Label ; let value_range = value . syntax () . text_range () ; let navs = navigation_target :: orig_range_with_focus_r (db , file_id , value_range , focus_range) ; navs . map (| (hir :: FileRangeWrapper { file_id , range } , focus_range) | { NavigationTarget :: from_syntax (file_id , hir :: Symbol :: intern ("<expr>") , focus_range , range , kind ,) }) }
    };
}

expr_to_nav!()