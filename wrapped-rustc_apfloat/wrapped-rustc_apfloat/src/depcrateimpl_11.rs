// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T > StatusAnd < T > { # [doc = " Keep the existing status but apply a transformation to `value`."] pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> StatusAnd < U > { StatusAnd { status : self . status , value : f (self . value) , } } }
};
}
