// Generated macro for macro_10768 (macro)
macro_rules! Depcrate_unit_return_expecting_ordmacro_10768 {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"macro_10768"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions that expect closures of type"] # [doc = " Fn(...) -> Ord where the implemented closure returns the unit type."] # [doc = " The lint also suggests to remove the semi-colon at the end of the statement if present."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Likely, returning the unit type is unintentional, and"] # [doc = " could simply be caused by an extra semi-colon. Since () implements Ord"] # [doc = " it doesn't cause a compilation error."] # [doc = " This is the same reasoning behind the unit_cmp lint."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If returning unit is intentional, then there is no"] # [doc = " way of specifying this without triggering needless_return lint"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut twins = vec![(1, 1), (2, 2)];"] # [doc = " twins.sort_by_key(|x| { x.1; });"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub UNIT_RETURN_EXPECTING_ORD , correctness , "fn arguments of type Fn(...) -> Ord returning the unit type ()." }
};
}
