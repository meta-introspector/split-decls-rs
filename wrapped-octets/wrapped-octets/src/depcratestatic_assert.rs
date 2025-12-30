// Generated macro for static_assert (macro)
macro_rules! Depcratestatic_assert {
() => {
// Module: crate
// Provides: {"static_assert"}
// Dependencies: {}
# [doc = " Helper macro that asserts at compile time. It requires that"] # [doc = " `cond` is a const expression."] macro_rules ! static_assert { ($ cond : expr) => { { const _ : () = assert ! ($ cond) ; } } ; }
};
}
