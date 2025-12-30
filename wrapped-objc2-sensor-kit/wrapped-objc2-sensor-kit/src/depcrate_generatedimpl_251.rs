// Generated macro for impl_251 (impl)
macro_rules! Depcrate_generatedimpl_251 {
() => {
// Module: crate::generated
// Provides: {"impl_251"}
// Dependencies: {}
impl SRMediaEvent { extern_methods ! (# [doc = " Unique media identifier"] # [doc = ""] # [doc = ""] # [doc = " Unique media identifier to track a specific media object."] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (mediaIdentifier))] # [unsafe (method_family = none)] pub unsafe fn mediaIdentifier (& self) -> Retained < NSString >; # [doc = " Type of the event"] # [doc = ""] # [doc = ""] # [doc = " Type of media event (e.g., media has been displayed on a screen)."] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (eventType))] # [unsafe (method_family = none)] pub unsafe fn eventType (& self) -> SRMediaEventType ;) ; }
};
}
