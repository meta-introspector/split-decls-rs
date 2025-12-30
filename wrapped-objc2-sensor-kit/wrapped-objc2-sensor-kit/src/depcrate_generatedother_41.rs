// Generated macro for other_41 (other)
macro_rules! Depcrate_generatedother_41 {
() => {
// Module: crate::generated
// Provides: {"other_41"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Convert a mach_continuous_time to an SRAbsoluteTime."] # [doc = ""] # [doc = " Because mach_continuous_time is volatile and hardware specific, the"] # [doc = " mach_continuous_time must originate from the same device and boot session"] # [doc = " that SRAbsoluteTimeFromContinuousTime() is called from."] # [doc = " The return value for mach_continuous_times spanning boot sessions or devices"] # [doc = " is undefined."] # [cfg (feature = "objc2-core-foundation")] pub fn SRAbsoluteTimeFromContinuousTime (cont : u64) -> SRAbsoluteTime ; }
};
}
