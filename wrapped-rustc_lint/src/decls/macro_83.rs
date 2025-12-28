macro_rules! macro_83 {
    () => {
        declare_lint_pass ! (# [doc = " Check for uses of edition keywords used as an identifier."] KeywordIdents => [KEYWORD_IDENTS_2018 , KEYWORD_IDENTS_2024]) ;
    };
}

macro_83!();