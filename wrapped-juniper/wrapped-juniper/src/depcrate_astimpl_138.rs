// Generated macro for impl_138 (impl)
macro_rules! Depcrate_astimpl_138 {
() => {
// Module: crate::ast
// Provides: {"impl_138"}
// Dependencies: {}
impl < N1 , N2 , M1 , M2 > PartialEq < Type < N2 , M2 > > for Type < N1 , M1 > where N1 : AsRef < str > , N2 : AsRef < str > , M1 : AsRef < [TypeModifier] > , M2 : AsRef < [TypeModifier] > , { fn eq (& self , other : & Type < N2 , M2 >) -> bool { self . name . as_ref () == other . name . as_ref () && self . modifiers . as_ref () == other . modifiers . as_ref () } }
};
}
