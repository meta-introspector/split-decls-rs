// Generated macro for impl_35 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_35 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_35"}
// Dependencies: {}
impl AllocBytes for MiriAllocBytes { type AllocParams = MiriAllocParams ; fn from_bytes < 'a > (slice : impl Into < Cow < 'a , [u8] > > , align : Align , params : MiriAllocParams ,) -> Self { let slice = slice . into () ; let size = slice . len () ; let align = align . bytes () ; let alloc_fn = | layout , params : & MiriAllocParams | unsafe { match params { MiriAllocParams :: Global => alloc :: alloc (layout) , MiriAllocParams :: Isolated (alloc) => alloc . borrow_mut () . alloc (layout) , } } ; let alloc_bytes = MiriAllocBytes :: alloc_with (size . to_u64 () , align , params , alloc_fn) . unwrap_or_else (| () | { panic ! ("Miri ran out of memory: cannot create allocation of {size} bytes") }) ; unsafe { alloc_bytes . ptr . copy_from (slice . as_ptr () , size) } ; alloc_bytes } fn zeroed (size : Size , align : Align , params : MiriAllocParams) -> Option < Self > { let size = size . bytes () ; let align = align . bytes () ; let alloc_fn = | layout , params : & MiriAllocParams | unsafe { match params { MiriAllocParams :: Global => alloc :: alloc_zeroed (layout) , MiriAllocParams :: Isolated (alloc) => alloc . borrow_mut () . alloc_zeroed (layout) , } } ; MiriAllocBytes :: alloc_with (size , align , params , alloc_fn) . ok () } fn as_mut_ptr (& mut self) -> * mut u8 { self . ptr } fn as_ptr (& self) -> * const u8 { self . ptr } }
};
}
