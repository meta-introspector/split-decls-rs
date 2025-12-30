// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < T , const N : usize > PartialEq < [T ; N] > for VecList < T > where T : PartialEq , { fn eq (& self , other : & [T ; N]) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
