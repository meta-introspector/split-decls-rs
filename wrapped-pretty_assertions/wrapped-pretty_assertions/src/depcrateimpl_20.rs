// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , TLeft , TRight > Comparison < 'a , TLeft , TRight > where TLeft : ? Sized , TRight : ? Sized , { # [doc = " Store two values to be compared in future."] # [doc = ""] # [doc = " Expensive diffing is deferred until calling `Debug::fmt`."] pub fn new (left : & 'a TLeft , right : & 'a TRight) -> Comparison < 'a , TLeft , TRight > { Comparison { left , right } } }
};
}
