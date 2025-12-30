// Generated macro for impl_313 (impl)
macro_rules! Depcrate_eventsimpl_313 {
() => {
// Module: crate::events
// Provides: {"impl_313"}
// Dependencies: {}
impl < 'a > Debug for BytesPI < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "BytesPI {{ content: ") ? ; write_cow_string (f , & self . content . buf) ? ; write ! (f , " }}") } }
};
}
