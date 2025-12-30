// Generated macro for check (function)
macro_rules! Depcrate_attrs_should_panic_without_expectcheck {
() => {
// Module: crate::attrs::should_panic_without_expect
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , attr : & Attribute) { if let AttrKind :: Normal (normal_attr) = & attr . kind { if let AttrArgs :: Eq { .. } = & normal_attr . item . args { return ; } if let AttrArgs :: Delimited (args) = & normal_attr . item . args && let mut tt_iter = args . tokens . iter () && let Some (TokenTree :: Token (Token { kind : TokenKind :: Ident (sym :: expected , _) , .. } , _ ,)) = tt_iter . next () && let Some (TokenTree :: Token (Token { kind : TokenKind :: Eq , .. } , _ ,)) = tt_iter . next () && let Some (TokenTree :: Token (Token { kind : TokenKind :: Literal (_) , .. } , _ ,)) = tt_iter . next () { return ; } span_lint_and_sugg (cx , SHOULD_PANIC_WITHOUT_EXPECT , attr . span , "#[should_panic] attribute without a reason" , "consider specifying the expected panic" , "#[should_panic(expected = /* panic message */)]" . into () , Applicability :: HasPlaceholders ,) ; } }
};
}
