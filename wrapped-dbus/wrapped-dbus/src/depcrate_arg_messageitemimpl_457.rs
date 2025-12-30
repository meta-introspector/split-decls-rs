// Generated macro for impl_457 (impl)
macro_rules! Depcrate_arg_messageitemimpl_457 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_457"}
// Dependencies: {}
impl MessageItemDict { # [doc = " Creates a new dict where every key and value elements have the supplied signature."] pub fn new (v : Vec < (MessageItem , MessageItem) > , keysig : Signature < 'static > , valuesig : Signature < 'static >) -> Result < MessageItemDict , ArrayError > { let sig = Signature :: from (format ! ("a{{{}{}}}" , keysig , valuesig)) ; let a = MessageItemDict { v : v , sig : sig } ; for (k , v) in & a . v { if keysig != k . signature () || valuesig != v . signature () { return Err (ArrayError :: DifferentElementTypes) ; } } Ok (a) } fn element_signature (& self) -> & CStr { let z = & self . sig . as_cstr () . to_bytes_with_nul () [1 ..] ; unsafe { CStr :: from_bytes_with_nul_unchecked (z) } } # [doc = " Signature of array (full array signature)"] pub fn signature (& self) -> & Signature < 'static > { & self . sig } # [doc = " Consumes the MessageItemDict in order to allow you to modify the individual items of the dict."] pub fn into_vec (self) -> Vec < (MessageItem , MessageItem) > { self . v } }
};
}
