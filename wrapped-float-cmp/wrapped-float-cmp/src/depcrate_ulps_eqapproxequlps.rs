// Generated macro for ApproxEqUlps (trait)
macro_rules! Depcrate_ulps_eqApproxEqUlps {
() => {
// Module: crate::ulps_eq
// Provides: {"ApproxEqUlps"}
// Dependencies: {}
# [doc = " ApproxEqUlps is a trait for approximate equality comparisons."] # [doc = " The associated type Flt is a floating point type which implements Ulps, and is"] # [doc = " required so that this trait can be implemented for compound types (e.g. vectors),"] # [doc = " not just for the floats themselves."] pub trait ApproxEqUlps { type Flt : Ulps ; # [doc = " This method tests for `self` and `other` values to be approximately equal"] # [doc = " within ULPs (Units of Least Precision) floating point representations."] # [doc = " Differing signs are always unequal with this method, and zeroes are only"] # [doc = " equal to zeroes. Use approx_eq() from the ApproxEq trait if that is more"] # [doc = " appropriate."] fn approx_eq_ulps (& self , other : & Self , ulps : < Self :: Flt as Ulps > :: U) -> bool ; # [doc = " This method tests for `self` and `other` values to be not approximately"] # [doc = " equal within ULPs (Units of Least Precision) floating point representations."] # [doc = " Differing signs are always unequal with this method, and zeroes are only"] # [doc = " equal to zeroes. Use approx_eq() from the ApproxEq trait if that is more"] # [doc = " appropriate."] # [inline] fn approx_ne_ulps (& self , other : & Self , ulps : < Self :: Flt as Ulps > :: U) -> bool { ! self . approx_eq_ulps (other , ulps) } }
};
}
