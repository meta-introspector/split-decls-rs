// Generated macro for impl_5 (impl)
macro_rules! Depcrate_allocimpl_5 {
() => {
// Module: crate::alloc
// Provides: {"impl_5"}
// Dependencies: {}
impl < T > BakeSize for alloc :: borrow :: Cow < '_ , T > where T : ? Sized + ToOwned , for < 'a > & 'a T : BakeSize , { fn borrows_size (& self) -> usize { (& * * self) . borrows_size () } }
};
}
