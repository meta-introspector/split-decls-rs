// Generated macro for impl_840 (impl)
macro_rules! Depcrate_ptrimpl_840 {
() => {
// Module: crate::ptr
// Provides: {"impl_840"}
// Dependencies: {}
impl < T > ConstPointer < 'static , T > { pub unsafe fn new_static (ptr : * const T) -> Result < Self , () > { if ptr . is_null () { return Err (()) ; } Ok (ConstPointer { ptr , _lifetime : PhantomData , }) } }
};
}
