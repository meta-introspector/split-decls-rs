// Generated macro for OS (const)
macro_rules! Depcrate_core_build_steps_toolstateOS {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"OS"}
// Dependencies: {}
# [cfg (all (not (target_os = "linux") , not (windows)))] const OS : Option < & str > = None ;
};
}
