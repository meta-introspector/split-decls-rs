// Generated macro for assert_biteq (macro)
macro_rules! Depcrate_math_support_macrosassert_biteq {
() => {
// Module: crate::math::support::macros
// Provides: {"assert_biteq"}
// Dependencies: {}
# [doc = " Assert `F::biteq` with better messages."] # [cfg (test)] macro_rules ! assert_biteq { ($ left : expr , $ right : expr , $ ($ tt : tt) *) => { { let l = $ left ; let r = $ right ; let bits = $ crate :: support :: Int :: leading_zeros (l . to_bits () - l . to_bits ()) ; assert ! ($ crate :: support :: Float :: biteq (l , r) , "{}\nl: {l:?} ({lb:#0width$x} {lh})\nr: {r:?} ({rb:#0width$x} {rh})" , format_args ! ($ ($ tt) *) , lb = l . to_bits () , lh = $ crate :: support :: Hexf (l) , rb = r . to_bits () , rh = $ crate :: support :: Hexf (r) , width = ((bits / 4) + 2) as usize ,) ; } } ; ($ left : expr , $ right : expr $ (,) ?) => { assert_biteq ! ($ left , $ right , "") } ; }
};
}
