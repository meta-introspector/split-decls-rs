// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bufimpl_22 {
() => {
// Module: crate::buf
// Provides: {"impl_22"}
// Dependencies: {}
impl < const SIZE : usize > fmt :: Write for WriteBuffer < SIZE > { fn write_str (& mut self , s : & str) -> fmt :: Result { let bytes = s . as_bytes () ; if let Some (buf) = self . buf . get_mut (self . len .. (self . len + bytes . len ())) { maybe_uninit_write_slice (buf , bytes) ; self . len += bytes . len () ; Ok (()) } else { Err (fmt :: Error) } } }
};
}
