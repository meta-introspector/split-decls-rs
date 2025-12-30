// Generated macro for _FORCE_DYNAMIC_DETECTION_HONORED (const)
macro_rules! Depcrate_cpu_FORCE_DYNAMIC_DETECTION_HONORED {
() => {
// Module: crate::cpu
// Provides: {"_FORCE_DYNAMIC_DETECTION_HONORED"}
// Dependencies: {}
# [allow (clippy :: assertions_on_constants , clippy :: bad_bit_mask)] const _FORCE_DYNAMIC_DETECTION_HONORED : () = assert ! ((CAPS_STATIC & featureflags :: FORCE_DYNAMIC_DETECTION) == 0) ;
};
}
