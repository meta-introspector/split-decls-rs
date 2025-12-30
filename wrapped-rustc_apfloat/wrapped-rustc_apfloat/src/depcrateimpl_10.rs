// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Status { # [doc = " Add a value to this status to create a [`StatusAnd`]."] pub fn and < T > (self , value : T) -> StatusAnd < T > { StatusAnd { status : self , value } } }
};
}
