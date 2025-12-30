// Generated macro for impl_161 (impl)
macro_rules! Depcrate_bytesimpl_161 {
() => {
// Module: crate::bytes
// Provides: {"impl_161"}
// Dependencies: {}
impl From < Box < [u8] > > for Bytes { fn from (slice : Box < [u8] >) -> Bytes { if slice . is_empty () { return Bytes :: new () ; } let len = slice . len () ; let ptr = Box :: into_raw (slice) as * mut u8 ; if ptr as usize & 0x1 == 0 { let data = ptr_map (ptr , | addr | addr | KIND_VEC) ; Bytes { ptr , len , data : AtomicPtr :: new (data . cast ()) , vtable : & PROMOTABLE_EVEN_VTABLE , } } else { Bytes { ptr , len , data : AtomicPtr :: new (ptr . cast ()) , vtable : & PROMOTABLE_ODD_VTABLE , } } } }
};
}
