// Generated macro for impl_176 (impl)
macro_rules! Depcrateimpl_176 {
() => {
// Module: crate
// Provides: {"impl_176"}
// Dependencies: {}
impl < T , U , const N : usize > PartialEq < [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & [U]) -> bool { self [..] == other [..] } }
};
}
