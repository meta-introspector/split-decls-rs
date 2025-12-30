// Generated macro for __auto_eq (macro)
macro_rules! Depcrate_matcher_support_auto_eq__auto_eq {
() => {
// Module: crate::matcher_support::auto_eq
// Provides: {"__auto_eq"}
// Dependencies: {}
# [doc = " Macro that wraps the expression with `eq(...)` if the expression is"] # [doc = " not a matcher."] # [doc = ""] # [doc = " This is useful to let users pass expected value to macro matchers like"] # [doc = " `field!` and `property!`."] # [doc = "`"] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc = " If you are interested in using it in your matcher, please file an issue to"] # [doc = " stabilize this."] # [macro_export] macro_rules ! __auto_eq { ($ e : expr) => { { # [allow (unused_imports)] use $ crate :: matcher_support :: __internal_unstable_do_not_depend_on_these :: ExpectedKind as _ ; match $ e { expected => { $ crate :: matcher_support :: __internal_unstable_do_not_depend_on_these :: Wrapper (& expected ,) . kind () . matcher (expected) } } } } ; }
};
}
