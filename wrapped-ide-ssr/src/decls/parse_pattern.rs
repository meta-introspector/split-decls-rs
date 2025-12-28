macro_rules! deps {
    () => {
        PatternElement!();
        Token!();
        SsrError!();
        Placeholder!();
    };
}

macro_rules! parse_pattern {
    () => {
        deps!();
        # [doc = " Returns `pattern_str`, parsed as a search or replace pattern. If `remove_whitespace` is true,"] # [doc = " then any whitespace tokens will be removed, which we do for the search pattern, but not for the"] # [doc = " replace pattern."] fn parse_pattern (pattern_str : & str) -> Result < Vec < PatternElement > , SsrError > { let mut res = Vec :: new () ; let mut placeholder_names = FxHashSet :: default () ; let mut tokens = tokenize (pattern_str) ? . into_iter () ; while let Some (token) = tokens . next () { if token . kind == T ! [$] { let placeholder = parse_placeholder (& mut tokens) ? ; if ! placeholder_names . insert (placeholder . ident . clone ()) { bail ! ("Placeholder `{}` repeats more than once" , placeholder . ident) ; } res . push (PatternElement :: Placeholder (placeholder)) ; } else { res . push (PatternElement :: Token (token)) ; } } Ok (res) }
    };
}

parse_pattern!()