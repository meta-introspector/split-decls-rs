// Generated macro for impl_70 (impl)
macro_rules! Depcrate_borrowimpl_70 {
() => {
// Module: crate::borrow
// Provides: {"impl_70"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized + ToOwned > Clone for Cow < '_ , B > { fn clone (& self) -> Self { match * self { Borrowed (b) => Borrowed (b) , Owned (ref o) => { let b : & B = o . borrow () ; Owned (b . to_owned ()) } } } fn clone_from (& mut self , source : & Self) { match (self , source) { (& mut Owned (ref mut dest) , & Owned (ref o)) => o . borrow () . clone_into (dest) , (t , s) => * t = s . clone () , } } }
};
}
