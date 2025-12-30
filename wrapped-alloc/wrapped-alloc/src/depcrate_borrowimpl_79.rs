// Generated macro for impl_79 (impl)
macro_rules! Depcrate_borrowimpl_79 {
() => {
// Module: crate::borrow
// Provides: {"impl_79"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , B : ? Sized > PartialOrd for Cow < 'a , B > where B : PartialOrd + ToOwned , { # [inline] fn partial_cmp (& self , other : & Cow < 'a , B >) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
