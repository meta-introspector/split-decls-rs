// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
impl < T , U , const N : usize > PartialEq < & [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & & [U]) -> bool { self [..] == other [..] } }
};
}
