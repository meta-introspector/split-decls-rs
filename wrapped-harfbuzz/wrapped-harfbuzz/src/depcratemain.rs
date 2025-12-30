// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main () { let funcs = compiled_funcs () ; let mut b = Buffer :: with ("Hello World") ; b . set_unicode_funcs (& funcs) ; b . guess_segment_properties () ; assert_eq ! (b . get_direction () , Direction :: LTR) ; assert_eq ! (b . get_script () , sys :: HB_SCRIPT_LATIN) ; let mut b = Buffer :: with ("مساء الخير") ; b . set_unicode_funcs (& funcs) ; b . guess_segment_properties () ; assert_eq ! (b . get_direction () , Direction :: RTL) ; assert_eq ! (b . get_script () , sys :: HB_SCRIPT_ARABIC) ; }
};
}
