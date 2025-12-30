// Generated macro for MemLabel (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsMemLabel {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"MemLabel"}
// Dependencies: {}
# [doc = " A reference to some memory address."] # [derive (Clone , Debug)] pub enum MemLabel { # [doc = " An address in the code, a constant pool or jumptable, with relative"] # [doc = " offset from this instruction. This form must be used at emission time;"] # [doc = " see `memlabel_finalize()` for how other forms are lowered to this one."] PCRel (i32) , # [doc = " An address that refers to a label within a `MachBuffer`, for example a"] # [doc = " constant that lives in the pool at the end of the function."] Mach (MachLabel) , }
};
}
