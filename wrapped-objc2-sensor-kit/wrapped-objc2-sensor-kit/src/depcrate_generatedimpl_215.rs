// Generated macro for impl_215 (impl)
macro_rules! Depcrate_generatedimpl_215 {
() => {
// Module: crate::generated
// Provides: {"impl_215"}
// Dependencies: {}
impl SRWristDetection { extern_methods ! (# [unsafe (method (onWrist))] # [unsafe (method_family = none)] pub unsafe fn onWrist (& self) -> bool ; # [unsafe (method (wristLocation))] # [unsafe (method_family = none)] pub unsafe fn wristLocation (& self) -> SRWristLocation ; # [unsafe (method (crownOrientation))] # [unsafe (method_family = none)] pub unsafe fn crownOrientation (& self) -> SRCrownOrientation ; # [doc = " Start date of the recent on-wrist state."] # [doc = ""] # [doc = ""] # [doc = " - When the state changes from off-wrist to on-wrist, onWristDate would be updated to the current date, and offWristDate would remain the same."] # [doc = " - When the state changes from on-wrist to off-wrist, offWristDate would be updated to the current date, and onWristDate would remain the same."] # [unsafe (method (onWristDate))] # [unsafe (method_family = none)] pub unsafe fn onWristDate (& self) -> Option < Retained < NSDate >>; # [doc = " Start date of the recent off-wrist state."] # [doc = ""] # [doc = ""] # [doc = " - When the state changes from off-wrist to on-wrist, onWristDate would be updated to the current date, and offWristDate would remain the same."] # [doc = " - When the state changes from on-wrist to off-wrist, offWristDate would be updated to the current date, and onWristDate would remain the same."] # [unsafe (method (offWristDate))] # [unsafe (method_family = none)] pub unsafe fn offWristDate (& self) -> Option < Retained < NSDate >>;) ; }
};
}
