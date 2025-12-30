// Generated macro for MemInstType (struct)
macro_rules! Depcrate_isa_s390x_inst_emitMemInstType {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"MemInstType"}
// Dependencies: {}
# [doc = " Type(s) of memory instructions available for mem_finalize."] pub struct MemInstType { # [doc = " True if 12-bit unsigned displacement is supported."] pub have_d12 : bool , # [doc = " True if 20-bit signed displacement is supported."] pub have_d20 : bool , # [doc = " True if PC-relative addressing is supported (memory access)."] pub have_pcrel : bool , # [doc = " True if PC-relative addressing is supported (load address)."] pub have_unaligned_pcrel : bool , # [doc = " True if an index register is supported."] pub have_index : bool , }
};
}
