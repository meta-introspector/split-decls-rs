// Generated macro for impl_149 (impl)
macro_rules! Depcrate_errorsimpl_149 {
() => {
// Module: crate::errors
// Provides: {"impl_149"}
// Dependencies: {}
impl < T > CapacityError < T > { # [doc = " Create a new `CapacityError` from `element`."] pub const fn new (element : T) -> CapacityError < T > { CapacityError { element : element } } # [doc = " Extract the overflowing element"] pub fn element (self) -> T { self . element } # [doc = " Convert into a `CapacityError` that does not carry an element."] pub fn simplify (self) -> CapacityError { CapacityError { element : () } } }
};
}
