// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , T > PartialEq < & 'a [T] > for VecList < T > where T : PartialEq , { fn eq (& self , other : & & 'a [T]) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
