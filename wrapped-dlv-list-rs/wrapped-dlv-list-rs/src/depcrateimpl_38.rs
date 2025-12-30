// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > PartialEq < LinkedList < T > > for VecList < T > where T : PartialEq , { fn eq (& self , other : & LinkedList < T >) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
};
}
