// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > fmt :: Debug for Event < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_inner () { Some (inner) => { let notified_count = inner . notified . load (Ordering :: Relaxed) ; let total_count = match inner . list . try_total_listeners () { Some (total_count) => total_count , None => { return f . debug_tuple ("Event") . field (& format_args ! ("<locked>")) . finish () } } ; f . debug_struct ("Event") . field ("listeners_notified" , & notified_count) . field ("listeners_total" , & total_count) . finish () } None => f . debug_tuple ("Event") . field (& format_args ! ("<uninitialized>")) . finish () , } } }
};
}
