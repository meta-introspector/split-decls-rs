// Generated macro for ArchIndependentRegs (trait)
macro_rules! Depcrate_shims_native_lib_trace_parentArchIndependentRegs {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"ArchIndependentRegs"}
// Dependencies: {}
# [doc = " Allows us to get common arguments from the `user_regs_t` across architectures."] # [doc = " Normally this would land us ABI hell, but thankfully all of our usecases"] # [doc = " consist of functions with a small number of register-sized integer arguments."] # [doc = " See <https://man7.org/linux/man-pages/man2/syscall.2.html> for sources."] trait ArchIndependentRegs { # [doc = " Gets the address of the instruction pointer."] fn ip (& self) -> usize ; # [doc = " Set the instruction pointer; remember to also set the stack pointer, or"] # [doc = " else the stack might get messed up!"] fn set_ip (& mut self , ip : usize) ; # [doc = " Set the stack pointer, ideally to a zeroed-out area."] fn set_sp (& mut self , sp : usize) ; }
};
}
