// Generated macro for unerased_lint_store (function)
macro_rules! Depcrate_lateunerased_lint_store {
() => {
// Module: crate::late
// Provides: {"unerased_lint_store"}
// Dependencies: {}
# [doc = " Extract the [`LintStore`] from [`Session`]."] # [doc = ""] # [doc = " This function exists because [`Session::lint_store`] is type-erased."] pub fn unerased_lint_store (sess : & Session) -> & LintStore { let store : & dyn Any = sess . lint_store . as_deref () . unwrap () ; store . downcast_ref () . unwrap () }
};
}
