// Generated macro for impl_1205 (impl)
macro_rules! Depcrate_shims_native_lib_trace_parentimpl_1205 {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"impl_1205"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [rustfmt :: skip] impl ArchIndependentRegs for libc :: user_regs_struct { # [inline] fn ip (& self) -> usize { self . rip . try_into () . unwrap () } # [inline] fn set_ip (& mut self , ip : usize) { self . rip = ip . try_into () . unwrap () } # [inline] fn set_sp (& mut self , sp : usize) { self . rsp = sp . try_into () . unwrap () } }
};
}
