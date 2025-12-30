// Generated macro for impl_2555 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2555 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2555"}
// Dependencies: {}
impl XReg { # [doc = " Index of the first \"special\" register, or the end of which registers"] # [doc = " regalloc is allowed to use."] pub const SPECIAL_START : u8 = pulley_interpreter :: regs :: XReg :: SPECIAL_START ; # [doc = " Returns whether this is a \"special\" physical register for pulley."] pub fn is_special (& self) -> bool { match self . as_pulley () { Some (reg) => reg . is_special () , None => false , } } # [doc = " Returns the pulley-typed register, if this is a physical register."] pub fn as_pulley (& self) -> Option < pulley_interpreter :: XReg > { let enc = self . to_real_reg () ? . hw_enc () ; Some (pulley_interpreter :: XReg :: new (enc) . unwrap ()) } }
};
}
