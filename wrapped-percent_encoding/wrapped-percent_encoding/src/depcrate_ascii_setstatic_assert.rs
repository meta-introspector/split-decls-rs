// Generated macro for static_assert (macro)
macro_rules! Depcrate_ascii_setstatic_assert {
() => {
// Module: crate::ascii_set
// Provides: {"static_assert"}
// Dependencies: {}
macro_rules ! static_assert { ($ ($ bool : expr ,) +) => { fn _static_assert () { $ (let _ = mem :: transmute ::< [u8 ; $ bool as usize] , u8 >;) + } } }
};
}
