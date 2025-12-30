// Generated macro for get_disasm (function)
macro_rules! Depcrate_shims_native_lib_trace_parentget_disasm {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"get_disasm"}
// Dependencies: {}
# [doc = " Spawns a Capstone disassembler for the host architecture."] # [rustfmt :: skip] fn get_disasm () -> capstone :: Capstone { use capstone :: prelude :: * ; let cs_pre = Capstone :: new () ; { # [cfg (target_arch = "x86_64")] { cs_pre . x86 () . mode (arch :: x86 :: ArchMode :: Mode64) } # [cfg (target_arch = "x86")] { cs_pre . x86 () . mode (arch :: x86 :: ArchMode :: Mode32) } } . detail (true) . build () . unwrap () }
};
}
