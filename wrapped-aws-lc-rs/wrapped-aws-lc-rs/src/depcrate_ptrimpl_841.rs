// Generated macro for impl_841 (impl)
macro_rules! Depcrate_ptrimpl_841 {
() => {
// Module: crate::ptr
// Provides: {"impl_841"}
// Dependencies: {}
impl < T > ConstPointer < '_ , T > { pub fn project_const_lifetime < 'a , C > (& 'a self , f : unsafe fn (& 'a Self) -> * const C ,) -> Result < ConstPointer < 'a , C > , () > { let ptr = unsafe { f (self) } ; if ptr . is_null () { return Err (()) ; } Ok (ConstPointer { ptr , _lifetime : PhantomData , }) } }
};
}
