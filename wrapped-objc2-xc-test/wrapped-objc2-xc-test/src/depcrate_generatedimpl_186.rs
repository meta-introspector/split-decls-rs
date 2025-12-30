// Generated macro for impl_186 (impl)
macro_rules! Depcrate_generatedimpl_186 {
() => {
// Module: crate::generated
// Provides: {"impl_186"}
// Dependencies: {}
impl XCTPerformanceMeasurementTimestamp { extern_methods ! (# [doc = " The timestamp recorded using mach_absolute_time()."] # [unsafe (method (absoluteTime))] # [unsafe (method_family = none)] pub fn absoluteTime (& self) -> u64 ; # [doc = " Nanoseconds since an arbitrary point, does not increment while the system is asleep."] # [unsafe (method (absoluteTimeNanoSeconds))] # [unsafe (method_family = none)] pub fn absoluteTimeNanoSeconds (& self) -> u64 ; # [doc = " The timestamp recorded using an NSDate."] # [unsafe (method (date))] # [unsafe (method_family = none)] pub fn date (& self) -> Retained < NSDate >; # [doc = " Initializes an object with the given mach absolute time and NSDate instance."] # [unsafe (method (initWithAbsoluteTime : date :))] # [unsafe (method_family = init)] pub fn initWithAbsoluteTime_date (this : Allocated < Self >, absolute_time : u64 , date : & NSDate ,) -> Retained < Self >; # [doc = " Initializes an object which represents a timestamp at the current time."] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >;) ; }
};
}
