// Generated macro for impl_72 (impl)
macro_rules! Depcrate_borrowimpl_72 {
() => {
// Module: crate::borrow
// Provides: {"impl_72"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized + ToOwned > Deref for Cow < '_ , B > where B :: Owned : Borrow < B > , { type Target = B ; fn deref (& self) -> & B { match * self { Borrowed (borrowed) => borrowed , Owned (ref owned) => owned . borrow () , } } }
};
}
