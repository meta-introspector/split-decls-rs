// Generated macro for impl_183 (impl)
macro_rules! Depcrate_test_traitsimpl_183 {
() => {
// Module: crate::test_traits
// Provides: {"impl_183"}
// Dependencies: {}
impl < T1 , T2 , T3 , R > TupleCall < fn (T1 , T2 , T3) -> R > for (T1 , T2 , T3) where T1 : fmt :: Debug , T2 : fmt :: Debug , T3 : fmt :: Debug , { type Output = R ; fn call (self , f : fn (T1 , T2 , T3) -> R) -> Self :: Output { f (self . 0 , self . 1 , self . 2) } }
};
}
