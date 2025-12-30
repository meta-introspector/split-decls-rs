// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > PartialEq for VecList < T > where T : PartialEq , { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
};
}
