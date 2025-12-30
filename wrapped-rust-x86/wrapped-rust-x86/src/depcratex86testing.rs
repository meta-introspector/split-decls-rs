// Generated macro for x86testing (module)
macro_rules! Depcratex86testing {
() => {
// Module: crate
// Provides: {"x86testing"}
// Dependencies: {}
# [cfg (all (test , feature = "vmtest"))] mod x86testing { use super :: * ; use x86test :: * ; # [x86test (should_halt)] fn should_halt () { unsafe { halt () } } # [x86test] fn should_not_halt () { } }
};
}
