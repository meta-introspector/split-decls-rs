// Generated macro for AMO (enum)
macro_rules! Depcrate_isa_riscv64_inst_argsAMO {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"AMO"}
// Dependencies: {}
# [doc = "Atomic Memory ordering."] # [derive (Copy , Clone , Debug)] pub enum AMO { Relax = 0b00 , Release = 0b01 , Acquire = 0b10 , SeqCst = 0b11 , }
};
}
