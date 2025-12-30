// Generated macro for InterruptCause (enum)
macro_rules! Depcrate_frame_parserInterruptCause {
() => {
// Module: crate::frame_parser
// Provides: {"InterruptCause"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] # [doc = " The reason that frame parsing was interrupted."] pub enum InterruptCause { FinBit , ResetStream (u64) , }
};
}
