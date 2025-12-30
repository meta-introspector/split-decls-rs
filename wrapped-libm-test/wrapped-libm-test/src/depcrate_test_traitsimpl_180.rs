// Generated macro for impl_180 (impl)
macro_rules! Depcrate_test_traitsimpl_180 {
() => {
// Module: crate::test_traits
// Provides: {"impl_180"}
// Dependencies: {}
impl < T1 , R > TupleCall < fn (T1) -> R > for (T1 ,) where T1 : fmt :: Debug , { type Output = R ; fn call (self , f : fn (T1) -> R) -> Self :: Output { f (self . 0) } }
};
}
