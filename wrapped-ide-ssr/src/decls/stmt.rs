macro_rules! stmt {
    () => {
        pub (crate) fn stmt (s : & str) -> Result < SyntaxNode , () > { let template = "const _: () = { {}; };" ; let input = template . replace ("{}" , s) ; let parse = syntax :: SourceFile :: parse (& input , syntax :: Edition :: CURRENT) ; if ! parse . errors () . is_empty () { return Err (()) ; } let mut node = parse . tree () . syntax () . descendants () . skip (2) . find_map (ast :: Stmt :: cast) . ok_or (()) ? ; if ! s . ends_with (';') && node . to_string () . ends_with (';') { node = node . clone_for_update () ; if let Some (it) = node . syntax () . last_token () { it . detach () } } if node . to_string () != s { return Err (()) ; } Ok (node . syntax () . clone_subtree ()) }
    };
}

stmt!()