// Generated macro for impl_172 (impl)
macro_rules! Depcrateimpl_172 {
() => {
// Module: crate
// Provides: {"impl_172"}
// Dependencies: {}
impl < T , U , const N : usize , const M : usize > PartialEq < SmallVec < U , M > > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & SmallVec < U , M >) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
