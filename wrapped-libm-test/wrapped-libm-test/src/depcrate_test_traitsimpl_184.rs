// Generated macro for impl_184 (impl)
macro_rules! Depcrate_test_traitsimpl_184 {
() => {
// Module: crate::test_traits
// Provides: {"impl_184"}
// Dependencies: {}
impl < T1 , T2 , T3 , R > TupleCall < fn (T1 , T2 , & mut T3) -> R > for (T1 , T2) where T1 : fmt :: Debug , T2 : fmt :: Debug , T3 : fmt :: Debug + Default , { type Output = (R , T3) ; fn call (self , f : fn (T1 , T2 , & mut T3) -> R) -> Self :: Output { let mut t3 = T3 :: default () ; (f (self . 0 , self . 1 , & mut t3) , t3) } }
};
}
