// Generated macro for impl_172 (impl)
macro_rules! Depcrate_ptrimpl_172 {
() => {
// Module: crate::ptr
// Provides: {"impl_172"}
// Dependencies: {}
impl < T > Own < T > where T : ? Sized , { pub fn new (ptr : Box < T >) -> Self { Own { ptr : unsafe { NonNull :: new_unchecked (Box :: into_raw (ptr)) } , } } pub fn cast < U : CastTo > (self) -> Own < U :: Target > { Own { ptr : self . ptr . cast () , } } pub unsafe fn boxed (self) -> Box < T > { unsafe { Box :: from_raw (self . ptr . as_ptr ()) } } pub fn by_ref (& self) -> Ref < T > { Ref { ptr : self . ptr , lifetime : PhantomData , } } pub fn by_mut (& mut self) -> Mut < T > { Mut { ptr : self . ptr , lifetime : PhantomData , } } }
};
}
