// Generated macro for impl_222 (impl)
macro_rules! Depcrateimpl_222 {
() => {
// Module: crate
// Provides: {"impl_222"}
// Dependencies: {}
impl < const N : usize > IntoArrayLength for Const < N > where Const < N > : ToUInt , typenum :: U < N > : ArrayLength , { type ArrayLength = typenum :: U < N > ; }
};
}
