// Generated macro for LUMPY_PACING_MIN_BANDWIDTH_KBPS (const)
macro_rules! Depcrate_recovery_gcongestion_pacerLUMPY_PACING_MIN_BANDWIDTH_KBPS {
() => {
// Module: crate::recovery::gcongestion::pacer
// Provides: {"LUMPY_PACING_MIN_BANDWIDTH_KBPS"}
// Dependencies: {}
# [doc = " The minimum estimated client bandwidth below which the pacing sender will"] # [doc = " not allow bursts."] const LUMPY_PACING_MIN_BANDWIDTH_KBPS : Bandwidth = Bandwidth :: from_kbits_per_second (1_200) ;
};
}
