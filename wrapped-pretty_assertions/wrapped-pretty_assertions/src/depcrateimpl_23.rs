// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , TLeft , TRight > StrComparison < 'a , TLeft , TRight > where TLeft : AsRef < str > + ? Sized , TRight : AsRef < str > + ? Sized , { # [doc = " Store two values to be compared in future."] # [doc = ""] # [doc = " Expensive diffing is deferred until calling `Debug::fmt`."] pub fn new (left : & 'a TLeft , right : & 'a TRight) -> StrComparison < 'a , TLeft , TRight > { StrComparison { left , right } } }
};
}
