// Generated macro for impl_300 (impl)
macro_rules! Depcrate_eventsimpl_300 {
() => {
// Module: crate::events
// Provides: {"impl_300"}
// Dependencies: {}
impl < 'a > Debug for BytesText < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "BytesText {{ content: ") ? ; write_cow_string (f , & self . content) ? ; write ! (f , " }}") } }
};
}
