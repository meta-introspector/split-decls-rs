// Generated macro for impl_81 (impl)
macro_rules! Depcrate_borrowimpl_81 {
() => {
// Module: crate::borrow
// Provides: {"impl_81"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized > fmt :: Display for Cow < '_ , B > where B : fmt :: Display + ToOwned < Owned : fmt :: Display > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Borrowed (ref b) => fmt :: Display :: fmt (b , f) , Owned (ref o) => fmt :: Display :: fmt (o , f) , } } }
};
}
