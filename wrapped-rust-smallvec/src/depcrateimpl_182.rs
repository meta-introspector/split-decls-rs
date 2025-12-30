// Generated macro for impl_182 (impl)
macro_rules! Depcrateimpl_182 {
() => {
// Module: crate
// Provides: {"impl_182"}
// Dependencies: {}
impl < T , const N : usize > Borrow < [T] > for SmallVec < T , N > { # [inline] fn borrow (& self) -> & [T] { self . as_slice () } }
};
}
