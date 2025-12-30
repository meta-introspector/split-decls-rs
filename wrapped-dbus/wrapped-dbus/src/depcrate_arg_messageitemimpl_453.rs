// Generated macro for impl_453 (impl)
macro_rules! Depcrate_arg_messageitemimpl_453 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_453"}
// Dependencies: {}
impl MessageItemArray { # [doc = " Creates a new array where every element has the supplied signature."] # [doc = ""] # [doc = " Signature is the full array signature, not the signature of the element."] pub fn new (v : Vec < MessageItem > , sig : Signature < 'static >) -> Result < MessageItemArray , ArrayError > { let a = MessageItemArray { v : v , sig : sig } ; if a . sig . as_bytes () [0] != ffi :: DBUS_TYPE_ARRAY as u8 { return Err (ArrayError :: InvalidSignature) } { let esig = a . element_signature () ; for i in & a . v { if i . signature () . as_cstr () != esig { return Err (ArrayError :: DifferentElementTypes) } } } Ok (a) } fn element_signature (& self) -> & CStr { let z = & self . sig . as_cstr () . to_bytes_with_nul () [1 ..] ; unsafe { CStr :: from_bytes_with_nul_unchecked (z) } } fn make_sig (m : & MessageItem) -> Signature < 'static > { Signature :: new (format ! ("a{}" , m . signature ())) . unwrap () } # [doc = " Signature of array (full array signature)"] pub fn signature (& self) -> & Signature < 'static > { & self . sig } # [doc = " Consumes the MessageItemArray in order to allow you to modify the individual items of the array."] pub fn into_vec (self) -> Vec < MessageItem > { self . v } }
};
}
