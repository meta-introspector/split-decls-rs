// Generated macro for peel_derefs_adjustments (function)
macro_rules! Depcrate_autorefspeel_derefs_adjustments {
() => {
// Module: crate::autorefs
// Provides: {"peel_derefs_adjustments"}
// Dependencies: {}
# [doc = " Peel derefs adjustments until the last last element."] fn peel_derefs_adjustments < 'a > (mut adjs : & 'a [Adjustment < 'a >]) -> & 'a [Adjustment < 'a >] { while let [Adjustment { kind : Adjust :: Deref (_) , .. } , end @ ..] = adjs && ! end . is_empty () { adjs = end ; } adjs }
};
}
