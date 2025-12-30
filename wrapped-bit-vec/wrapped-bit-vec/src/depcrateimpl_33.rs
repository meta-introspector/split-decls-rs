// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < B : BitBlock > Index < usize > for BitVec < B > { type Output = bool ; # [inline] fn index (& self , i : usize) -> & bool { if self . get (i) . expect ("index out of bounds") { & TRUE } else { & FALSE } } }
};
}
