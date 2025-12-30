// Generated macro for impl_23 (impl)
macro_rules! Depcrate_generatedimpl_23 {
() => {
// Module: crate::generated
// Provides: {"impl_23"}
// Dependencies: {}
impl NFCReaderSession { extern_methods ! (# [unsafe (method (delegate))] # [unsafe (method_family = none)] pub unsafe fn delegate (& self) -> Option < Retained < AnyObject >>; # [doc = " YES if device supports NFC tag reading."] # [unsafe (method (readingAvailable))] # [unsafe (method_family = none)] pub unsafe fn readingAvailable () -> bool ; # [cfg (feature = "dispatch2")] # [doc = " The NFCReaderSessionDelegate delegate callbacks and the completion block handlers for tag operation will be dispatched on this queue."] # [unsafe (method (sessionQueue))] # [unsafe (method_family = none)] pub unsafe fn sessionQueue (& self) -> Retained < DispatchQueue >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >;) ; }
};
}
