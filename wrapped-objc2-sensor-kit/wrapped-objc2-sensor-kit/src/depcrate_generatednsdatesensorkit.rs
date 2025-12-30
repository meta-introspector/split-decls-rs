// Generated macro for NSDateSensorKit (trait)
macro_rules! Depcrate_generatedNSDateSensorKit {
() => {
// Module: crate::generated
// Provides: {"NSDateSensorKit"}
// Dependencies: {}
# [doc = " Category \"SensorKit\" on [`NSDate`]."] # [doc (alias = "SensorKit")] pub unsafe trait NSDateSensorKit : ClassType + Sized + private_NSDateSensorKit :: Sealed { extern_methods ! (# [cfg (feature = "objc2-core-foundation")] # [unsafe (method (dateWithSRAbsoluteTime :))] # [unsafe (method_family = none)] unsafe fn dateWithSRAbsoluteTime (time : SRAbsoluteTime) -> Retained < Self >; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (initWithSRAbsoluteTime :))] # [unsafe (method_family = init)] unsafe fn initWithSRAbsoluteTime (this : Allocated < Self >, time : SRAbsoluteTime ,) -> Retained < Self >; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (srAbsoluteTime))] # [unsafe (method_family = none)] unsafe fn srAbsoluteTime (& self) -> SRAbsoluteTime ;) ; }
};
}
