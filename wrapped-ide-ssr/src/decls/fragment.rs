macro_rules! fragment {
    () => {
        fn fragment < T : AstNode > (template : & str , s : & str) -> Result < SyntaxNode , () > { let s = s . trim () ; let input = template . replace ("{}" , s) ; let parse = syntax :: SourceFile :: parse (& input , syntax :: Edition :: CURRENT) ; if ! parse . errors () . is_empty () { return Err (()) ; } let node = parse . tree () . syntax () . descendants () . find_map (T :: cast) . ok_or (()) ? ; if node . syntax () . text () != s { return Err (()) ; } Ok (node . syntax () . clone_subtree ()) }
    };
}

fragment!();