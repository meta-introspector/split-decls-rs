// Generated macro for impl_289 (impl)
macro_rules! Depcrate_eventsimpl_289 {
() => {
// Module: crate::events
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'a > Debug for BytesStart < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "BytesStart {{ buf: ") ? ; write_cow_string (f , & self . buf) ? ; write ! (f , ", name_len: {} }}" , self . name_len) } }
};
}
