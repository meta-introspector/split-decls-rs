// Generated macro for Counter (enum)
macro_rules! Depcrate_perfcnt_intel_descriptionCounter {
() => {
// Module: crate::perfcnt::intel::description
// Provides: {"Counter"}
// Dependencies: {}
# [derive (Clone , Copy , Eq , PartialEq)] pub enum Counter { # [doc = " Bit-mask containing the fixed counters"] # [doc = " usable with the corresponding performance event."] Fixed (u8) , # [doc = " Bit-mask containing the programmable counters"] # [doc = " usable with the corresponding performance event."] Programmable (u8) , }
};
}
