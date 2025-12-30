// Generated macro for LUMPY_PACING_SIZE (const)
macro_rules! Depcrate_recovery_gcongestion_pacerLUMPY_PACING_SIZE {
() => {
// Module: crate::recovery::gcongestion::pacer
// Provides: {"LUMPY_PACING_SIZE"}
// Dependencies: {}
# [doc = " Number of packets that the pacing sender allows in bursts during pacing."] # [doc = " This is ignored if a flow's estimated bandwidth is lower than 1200 kbps."] const LUMPY_PACING_SIZE : usize = 2 ;
};
}
