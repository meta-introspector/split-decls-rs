// Generated macro for impl_96 (impl)
macro_rules! Depcrate_impls_alloc_impl_96 {
() => {
// Module: crate::impls::alloc_
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , T > Format for alloc :: borrow :: Cow < 'a , [T] > where T : 'a + Format , [T] : alloc :: borrow :: ToOwned < Owned = alloc :: vec :: Vec < T > > , { delegate_format ! ([T] , self , self . as_ref ()) ; }
};
}
