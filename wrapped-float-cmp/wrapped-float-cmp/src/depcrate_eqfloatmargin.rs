// Generated macro for FloatMargin (trait)
macro_rules! Depcrate_eqFloatMargin {
() => {
// Module: crate::eq
// Provides: {"FloatMargin"}
// Dependencies: {}
# [doc = " A margin specifying a maximum distance two floating point values can be while"] # [doc = " still being considered equal enough."] pub trait FloatMargin : Copy + Default { # [doc = " A floating-point type used for epsilon values"] type F ; # [doc = " An integer type used for ulps values"] type I ; # [doc = " Zero margin"] fn zero () -> Self ; # [doc = " Set the epsilon value for this margin"] fn epsilon (self , epsilon : Self :: F) -> Self ; # [doc = " Set the ulps value for this margin"] fn ulps (self , ulps : Self :: I) -> Self ; }
};
}
