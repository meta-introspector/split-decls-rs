// Generated macro for impl_181 (impl)
macro_rules! Depcrate_test_traitsimpl_181 {
() => {
// Module: crate::test_traits
// Provides: {"impl_181"}
// Dependencies: {}
impl < T1 , T2 , R > TupleCall < fn (T1 , T2) -> R > for (T1 , T2) where T1 : fmt :: Debug , T2 : fmt :: Debug , { type Output = R ; fn call (self , f : fn (T1 , T2) -> R) -> Self :: Output { f (self . 0 , self . 1) } }
};
}
