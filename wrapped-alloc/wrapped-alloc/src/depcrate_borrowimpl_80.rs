// Generated macro for impl_80 (impl)
macro_rules! Depcrate_borrowimpl_80 {
() => {
// Module: crate::borrow
// Provides: {"impl_80"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized > fmt :: Debug for Cow < '_ , B > where B : fmt :: Debug + ToOwned < Owned : fmt :: Debug > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Borrowed (ref b) => fmt :: Debug :: fmt (b , f) , Owned (ref o) => fmt :: Debug :: fmt (o , f) , } } }
};
}
