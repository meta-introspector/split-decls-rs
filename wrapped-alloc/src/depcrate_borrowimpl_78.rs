// Generated macro for impl_78 (impl)
macro_rules! Depcrate_borrowimpl_78 {
() => {
// Module: crate::borrow
// Provides: {"impl_78"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , 'b , B : ? Sized , C : ? Sized > PartialEq < Cow < 'b , C > > for Cow < 'a , B > where B : PartialEq < C > + ToOwned , C : ToOwned , { # [inline] fn eq (& self , other : & Cow < 'b , C >) -> bool { PartialEq :: eq (& * * self , & * * other) } }
};
}
