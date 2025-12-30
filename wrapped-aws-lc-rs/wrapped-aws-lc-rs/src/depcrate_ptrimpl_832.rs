// Generated macro for impl_832 (impl)
macro_rules! Depcrate_ptrimpl_832 {
() => {
// Module: crate::ptr
// Provides: {"impl_832"}
// Dependencies: {}
impl < P : Pointer > ManagedPointer < P > { # [inline] pub fn as_const (& self) -> ConstPointer < '_ , P :: T > { self . into () } pub fn project_const_lifetime < 'a , C > (& 'a self , f : unsafe fn (& 'a Self) -> * const C ,) -> Result < ConstPointer < 'a , C > , () > { let ptr = unsafe { f (self) } ; if ptr . is_null () { return Err (()) ; } Ok (ConstPointer { ptr , _lifetime : PhantomData , }) } # [inline] pub unsafe fn as_mut_unsafe (& self) -> MutPointer < P :: T > { MutPointer { ptr : self . pointer . as_const_ptr () . cast_mut () , } } # [inline] pub fn as_mut (& mut self) -> MutPointer < P :: T > { MutPointer { ptr : self . pointer . as_mut_ptr () , } } }
};
}
