// Generated macro for impl_110 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_110 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_110"}
// Dependencies: {}
impl Invalidation { fn generate_diagnostic (& self) -> (String , SpanData) { let message = if matches ! (self . cause , InvalidationCause :: Retag (_ , RetagInfo { cause : RetagCause :: FnEntry , .. })) { format ! ("{:?} was later invalidated at offsets {:?} by a {} inside this call" , self . tag , self . range , self . cause) } else { format ! ("{:?} was later invalidated at offsets {:?} by a {}" , self . tag , self . range , self . cause) } ; (message , self . span . data ()) } }
};
}
