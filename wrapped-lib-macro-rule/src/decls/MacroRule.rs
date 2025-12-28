macro_rules! MacroRule {
    () => {
        pub enum MacroRule { # [doc = " A function-style rule, for use with `m!()`"] Func { lhs : Vec < MatcherLoc > , lhs_span : Span , rhs : TokenTree } , # [doc = " An attr rule, for use with `#[m]`"] Attr { unsafe_rule : bool , args : Vec < MatcherLoc > , args_span : Span , body : Vec < MatcherLoc > , body_span : Span , rhs : TokenTree , } , # [doc = " A derive rule, for use with `#[m]`"] Derive { body : Vec < MatcherLoc > , body_span : Span , rhs : TokenTree } , }
    };
}

MacroRule!();