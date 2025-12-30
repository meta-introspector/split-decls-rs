// Generated macro for CaptureRef (struct)
macro_rules! Depcrate_util_interpolateCaptureRef {
() => {
// Module: crate::util::interpolate
// Provides: {"CaptureRef"}
// Dependencies: {}
# [doc = " `CaptureRef` represents a reference to a capture group inside some text."] # [doc = " The reference is either a capture group name or a number."] # [doc = ""] # [doc = " It is also tagged with the position in the text following the"] # [doc = " capture reference."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] struct CaptureRef < 'a > { cap : Ref < 'a > , end : usize , }
};
}
