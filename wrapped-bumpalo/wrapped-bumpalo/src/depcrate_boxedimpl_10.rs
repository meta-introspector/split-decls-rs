// Generated macro for impl_10 (impl)
macro_rules! Depcrate_boxedimpl_10 {
() => {
// Module: crate::boxed
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , 'b , T : ? Sized + PartialEq > PartialEq < Box < 'b , T > > for Box < 'a , T > { # [inline] fn eq (& self , other : & Box < 'b , T >) -> bool { PartialEq :: eq (& * * self , & * * other) } # [inline] fn ne (& self , other : & Box < 'b , T >) -> bool { PartialEq :: ne (& * * self , & * * other) } }
};
}
