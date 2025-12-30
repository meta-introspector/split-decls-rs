// Generated macro for impl_182 (impl)
macro_rules! Depcrate_test_traitsimpl_182 {
() => {
// Module: crate::test_traits
// Provides: {"impl_182"}
// Dependencies: {}
impl < T1 , T2 , R > TupleCall < fn (T1 , & mut T2) -> R > for (T1 ,) where T1 : fmt :: Debug , T2 : fmt :: Debug + Default , { type Output = (R , T2) ; fn call (self , f : fn (T1 , & mut T2) -> R) -> Self :: Output { let mut t2 = T2 :: default () ; (f (self . 0 , & mut t2) , t2) } }
};
}
