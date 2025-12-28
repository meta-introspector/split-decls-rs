macro_rules! parse_as_kind {
    () => {
        fn parse_as_kind (code : & str , kind : SyntaxKind) -> Option < SyntaxNode > { if ast :: Expr :: can_cast (kind) && let Ok (expr) = fragments :: expr (code) { return Some (expr) ; } if ast :: Item :: can_cast (kind) && let Ok (item) = fragments :: item (code) { return Some (item) ; } None }
    };
}

parse_as_kind!()