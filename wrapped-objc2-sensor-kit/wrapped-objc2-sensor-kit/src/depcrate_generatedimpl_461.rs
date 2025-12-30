// Generated macro for impl_461 (impl)
macro_rules! Depcrate_generatedimpl_461 {
() => {
// Module: crate::generated
// Provides: {"impl_461"}
// Dependencies: {}
impl SRSleepSession { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [doc = " Start date of sleep session"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (startDate))] # [unsafe (method_family = none)] pub unsafe fn startDate (& self) -> Retained < NSDate >; # [doc = " Sleep session duration"] # [doc = ""] # [doc = ""] # [doc = " Equal to 0 if endReason is SRSleepSessionEndReasonNoEndEvent"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (duration))] # [unsafe (method_family = none)] pub unsafe fn duration (& self) -> NSTimeInterval ; # [doc = " Sleep session unique identifier"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (identifier))] # [unsafe (method_family = none)] pub unsafe fn identifier (& self) -> Retained < NSString >;) ; }
};
}
