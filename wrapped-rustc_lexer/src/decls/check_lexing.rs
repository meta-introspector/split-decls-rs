macro_rules! deps {
    () => {
        FrontmatterAllowed!();
    };
}

macro_rules! check_lexing {
    () => {
        deps!();
        fn check_lexing (src : & str , frontmatter_allowed : FrontmatterAllowed , expect : Expect) { let actual : String = tokenize (src , frontmatter_allowed) . map (| token | format ! ("{:?}\n" , token)) . collect () ; expect . assert_eq (& actual) }
    };
}

check_lexing!();