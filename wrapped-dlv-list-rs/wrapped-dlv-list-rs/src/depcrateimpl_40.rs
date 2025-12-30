// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < T > PartialEq < Vec < T > > for VecList < T > where T : PartialEq , { fn eq (& self , other : & Vec < T >) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
};
}
