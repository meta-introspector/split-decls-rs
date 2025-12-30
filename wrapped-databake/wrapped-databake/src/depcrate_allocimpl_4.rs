// Generated macro for impl_4 (impl)
macro_rules! Depcrate_allocimpl_4 {
() => {
// Module: crate::alloc
// Provides: {"impl_4"}
// Dependencies: {}
impl < T > Bake for alloc :: borrow :: Cow < '_ , T > where T : ? Sized + ToOwned , for < 'a > & 'a T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("alloc") ; let t = < & T as Bake > :: bake (& & * * self , ctx) ; quote ! { alloc :: borrow :: Cow :: Borrowed (# t) } } }
};
}
