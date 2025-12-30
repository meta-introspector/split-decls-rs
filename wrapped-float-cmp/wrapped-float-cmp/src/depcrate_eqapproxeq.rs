// Generated macro for ApproxEq (trait)
macro_rules! Depcrate_eqApproxEq {
() => {
// Module: crate::eq
// Provides: {"ApproxEq"}
// Dependencies: {}
# [doc = " A trait for approximate equality comparisons."] pub trait ApproxEq : Sized { # [doc = " This type type defines a margin within which two values are to be"] # [doc = " considered approximately equal. It must implement `Default` so that"] # [doc = " `approx_eq()` can be called on unknown types."] type Margin : FloatMargin ; # [doc = " This method tests that the `self` and `other` values are equal within `margin`"] # [doc = " of each other."] fn approx_eq < M : Into < Self :: Margin > > (self , other : Self , margin : M) -> bool ; # [doc = " This method tests that the `self` and `other` values are not within `margin`"] # [doc = " of each other."] fn approx_ne < M : Into < Self :: Margin > > (self , other : Self , margin : M) -> bool { ! self . approx_eq (other , margin) } }
};
}
