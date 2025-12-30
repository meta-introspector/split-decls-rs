// Generated macro for static_assert (macro)
macro_rules! Depcrate_utilsstatic_assert {
() => {
// Module: crate::utils
// Provides: {"static_assert"}
// Dependencies: {}
macro_rules ! static_assert { ($ cond : expr $ (,) ?) => { { let [()] = [() ; (true & $ cond) as usize] ; } } ; }
};
}
