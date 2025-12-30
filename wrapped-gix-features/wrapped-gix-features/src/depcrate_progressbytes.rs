// Generated macro for bytes (function)
macro_rules! Depcrate_progressbytes {
() => {
// Module: crate::progress
// Provides: {"bytes"}
// Dependencies: {}
# [doc = " A unit for displaying bytes with throughput and progress percentage."] # [cfg (not (feature = "progress-unit-bytes"))] pub fn bytes () -> Option < Unit > { Some (unit :: label_and_mode ("B" , unit :: display :: Mode :: with_throughput () . and_percentage () ,)) }
};
}
