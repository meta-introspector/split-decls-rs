// Generated macro for impl_185 (impl)
macro_rules! Depcrate_test_traitsimpl_185 {
() => {
// Module: crate::test_traits
// Provides: {"impl_185"}
// Dependencies: {}
impl < T1 , T2 , T3 > TupleCall < for < 'a > fn (T1 , & 'a mut T2 , & 'a mut T3) > for (T1 ,) where T1 : fmt :: Debug , T2 : fmt :: Debug + Default , T3 : fmt :: Debug + Default , { type Output = (T2 , T3) ; fn call (self , f : for < 'a > fn (T1 , & 'a mut T2 , & 'a mut T3)) -> Self :: Output { let mut t2 = T2 :: default () ; let mut t3 = T3 :: default () ; f (self . 0 , & mut t2 , & mut t3) ; (t2 , t3) } }
};
}
