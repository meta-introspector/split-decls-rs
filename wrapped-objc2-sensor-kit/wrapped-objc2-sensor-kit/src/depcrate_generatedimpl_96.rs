// Generated macro for impl_96 (impl)
macro_rules! Depcrate_generatedimpl_96 {
() => {
// Module: crate::generated
// Provides: {"impl_96"}
// Dependencies: {}
impl SRVisit { extern_methods ! (# [cfg (feature = "objc2-core-location")] # [doc = " The distance between the location of interest to home"] # [unsafe (method (distanceFromHome))] # [unsafe (method_family = none)] pub unsafe fn distanceFromHome (& self) -> CLLocationDistance ; # [doc = " The range of time the arrival to a location of interest occurred"] # [unsafe (method (arrivalDateInterval))] # [unsafe (method_family = none)] pub unsafe fn arrivalDateInterval (& self) -> Retained < NSDateInterval >; # [doc = " The range of time the departure from a location of interest occurred"] # [unsafe (method (departureDateInterval))] # [unsafe (method_family = none)] pub unsafe fn departureDateInterval (& self) -> Retained < NSDateInterval >; # [unsafe (method (locationCategory))] # [unsafe (method_family = none)] pub unsafe fn locationCategory (& self) -> SRLocationCategory ; # [doc = " An identifier for the location of interest."] # [doc = " This can be used to identify the same location regardless of type"] # [unsafe (method (identifier))] # [unsafe (method_family = none)] pub unsafe fn identifier (& self) -> Retained < NSUUID >;) ; }
};
}
