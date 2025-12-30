// Generated macro for impl_178 (impl)
macro_rules! Depcrateimpl_178 {
() => {
// Module: crate
// Provides: {"impl_178"}
// Dependencies: {}
impl < T , U , const N : usize > PartialEq < & mut [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & & mut [U]) -> bool { self [..] == other [..] } }
};
}
