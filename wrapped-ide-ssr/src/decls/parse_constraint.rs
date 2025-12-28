macro_rules! deps {
    () => {
        Constraint!();
        NodeKind!();
        Token!();
        SsrError!();
    };
}

macro_rules! parse_constraint {
    () => {
        deps!();
        fn parse_constraint (tokens : & mut std :: vec :: IntoIter < Token >) -> Result < Constraint , SsrError > { let constraint_type = tokens . next () . ok_or_else (| | SsrError :: new ("Found end of placeholder while looking for a constraint")) ? . text . to_string () ; match constraint_type . as_str () { "kind" => { expect_token (tokens , "(") ? ; let t = tokens . next () . ok_or_else (| | { SsrError :: new ("Unexpected end of constraint while looking for kind") }) ? ; if t . kind != SyntaxKind :: IDENT { bail ! ("Expected ident, found {:?} while parsing kind constraint" , t . kind) ; } expect_token (tokens , ")") ? ; Ok (Constraint :: Kind (NodeKind :: from (& t . text) ?)) } "not" => { expect_token (tokens , "(") ? ; let sub = parse_constraint (tokens) ? ; expect_token (tokens , ")") ? ; Ok (Constraint :: Not (Box :: new (sub))) } x => bail ! ("Unsupported constraint type '{}'" , x) , } }
    };
}

parse_constraint!()