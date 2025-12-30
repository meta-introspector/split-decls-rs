// Generated macro for Select (struct)
macro_rules! Depcrate_selectSelect {
() => {
// Module: crate::select
// Provides: {"Select"}
// Dependencies: {}
# [doc = " Future for the `select` combinator, waiting for one of two futures to"] # [doc = " complete."] # [doc = ""] # [doc = " This is created by this `Future::select` method."] pub struct Select < A , B > where A : Future , B : Future < Item = A :: Item , Error = A :: Error > { inner : Option < (Collapsed < A > , Collapsed < B >) > , }
};
}
