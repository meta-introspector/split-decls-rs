// Generated macro for impl_89 (impl)
macro_rules! Depcrate_history_bufimpl_89 {
() => {
// Module: crate::history_buf
// Provides: {"impl_89"}
// Dependencies: {}
impl < T , const N : usize > Clone for HistoryBuf < T , N > where T : Clone , { fn clone (& self) -> Self { let mut ret = Self :: new () ; for (new , old) in ret . data . borrow_mut () . iter_mut () . zip (self . as_slice ()) { new . write (old . clone ()) ; } ret . filled = self . filled ; ret . write_at = self . write_at ; ret } }
};
}
