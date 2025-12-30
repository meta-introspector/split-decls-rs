// Generated macro for impl_183 (impl)
macro_rules! Depcrateimpl_183 {
() => {
// Module: crate
// Provides: {"impl_183"}
// Dependencies: {}
impl < T , const N : usize > BorrowMut < [T] > for SmallVec < T , N > { # [inline] fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
};
}
