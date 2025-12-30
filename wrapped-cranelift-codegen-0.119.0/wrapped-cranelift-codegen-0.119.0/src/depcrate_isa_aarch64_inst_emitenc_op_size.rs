// Generated macro for enc_op_size (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_op_size {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_op_size"}
// Dependencies: {}
# [doc = " Set the size bit of an instruction."] fn enc_op_size (op : u32 , size : OperandSize) -> u32 { (op & ! (1 << 31)) | (size . sf_bit () << 31) }
};
}
