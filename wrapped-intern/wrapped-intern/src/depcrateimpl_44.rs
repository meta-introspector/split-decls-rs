// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
# [doc = " Compares interned `Ref`s using pointer equality."] impl < T : Internable > PartialEq for Interned < T > { # [inline] fn eq (& self , other : & Self) -> bool { Arc :: ptr_eq (& self . arc , & other . arc) } }
};
}
