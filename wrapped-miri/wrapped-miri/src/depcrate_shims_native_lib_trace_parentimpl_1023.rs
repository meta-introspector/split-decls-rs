// Generated macro for impl_1023 (impl)
macro_rules! Depcrate_shims_native_lib_trace_parentimpl_1023 {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"impl_1023"}
// Dependencies: {}
# [cfg (target_arch = "x86")] # [rustfmt :: skip] impl ArchIndependentRegs for libc :: user_regs_struct { # [inline] fn ip (& self) -> usize { self . eip . cast_unsigned () . try_into () . unwrap () } # [inline] fn set_ip (& mut self , ip : usize) { self . eip = ip . cast_signed () . try_into () . unwrap () } # [inline] fn set_sp (& mut self , sp : usize) { self . esp = sp . cast_signed () . try_into () . unwrap () } }
};
}
