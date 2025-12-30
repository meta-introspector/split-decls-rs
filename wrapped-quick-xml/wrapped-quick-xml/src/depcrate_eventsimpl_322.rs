// Generated macro for impl_322 (impl)
macro_rules! Depcrate_eventsimpl_322 {
() => {
// Module: crate::events
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'a > Debug for BytesRef < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "BytesRef {{ content: ") ? ; write_cow_string (f , & self . content) ? ; write ! (f , " }}") } }
};
}
