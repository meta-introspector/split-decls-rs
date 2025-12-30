// Generated macro for setup (function)
macro_rules! Depcrate_utils_jobsetup {
() => {
// Module: crate::utils::job
// Provides: {"setup"}
// Dependencies: {}
# [cfg (all (unix , not (target_os = "haiku")))] pub unsafe fn setup (build : & mut crate :: Build) { if build . config . low_priority { unsafe { libc :: setpriority (libc :: PRIO_PGRP as _ , 0 , 10) ; } } }
};
}
