macro_rules! Ranker {
    () => {
        pub struct Ranker < 'a > { pub kind : parser :: SyntaxKind , pub text : & 'a str , pub ident_kind : bool , }
    };
}

Ranker!();