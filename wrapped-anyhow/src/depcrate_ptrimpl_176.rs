// Generated macro for impl_176 (impl)
macro_rules! Depcrate_ptrimpl_176 {
() => {
// Module: crate::ptr
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a , T > Ref < 'a , T > where T : ? Sized , { pub fn new (ptr : & 'a T) -> Self { Ref { ptr : NonNull :: from (ptr) , lifetime : PhantomData , } } # [cfg (not (anyhow_no_ptr_addr_of))] pub fn from_raw (ptr : NonNull < T >) -> Self { Ref { ptr , lifetime : PhantomData , } } pub fn cast < U : CastTo > (self) -> Ref < 'a , U :: Target > { Ref { ptr : self . ptr . cast () , lifetime : PhantomData , } } # [cfg (not (anyhow_no_ptr_addr_of))] pub fn by_mut (self) -> Mut < 'a , T > { Mut { ptr : self . ptr , lifetime : PhantomData , } } # [cfg (not (anyhow_no_ptr_addr_of))] pub fn as_ptr (self) -> * const T { self . ptr . as_ptr () as * const T } pub unsafe fn deref (self) -> & 'a T { unsafe { & * self . ptr . as_ptr () } } }
};
}
