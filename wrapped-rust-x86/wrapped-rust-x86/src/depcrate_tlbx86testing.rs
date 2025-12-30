// Generated macro for x86testing (module)
macro_rules! Depcrate_tlbx86testing {
() => {
// Module: crate::tlb
// Provides: {"x86testing"}
// Dependencies: {}
# [cfg (all (test , feature = "vmtest"))] mod x86testing { use super :: * ; use x86test :: * ; # [x86test] fn check_flush_all () { unsafe { flush_all () ; } } # [x86test] fn check_flush () { unsafe { flush (0xdeadbeef) ; } } }
};
}
