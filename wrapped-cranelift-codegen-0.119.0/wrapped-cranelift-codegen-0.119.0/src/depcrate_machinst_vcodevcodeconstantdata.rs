// Generated macro for VCodeConstantData (enum)
macro_rules! Depcrate_machinst_vcodeVCodeConstantData {
() => {
// Module: crate::machinst::vcode
// Provides: {"VCodeConstantData"}
// Dependencies: {}
# [doc = " Identify the different types of constant that can be inserted into [VCodeConstants]. Tracking"] # [doc = " these separately instead of as raw byte buffers allows us to avoid some duplication."] pub enum VCodeConstantData { # [doc = " A constant already present in the Cranelift IR"] # [doc = " [ConstantPool](crate::ir::constant::ConstantPool)."] Pool (Constant , ConstantData) , # [doc = " A reference to a well-known constant value that is statically encoded within the compiler."] WellKnown (& 'static [u8]) , # [doc = " A constant value generated during lowering; the value may depend on the instruction context"] # [doc = " which makes it difficult to de-duplicate--if possible, use other variants."] Generated (ConstantData) , # [doc = " A constant of at most 64 bits. These are deduplicated as"] # [doc = " well. Stored as a fixed-size array of `u8` so that we do not"] # [doc = " encounter endianness problems when cross-compiling."] U64 ([u8 ; 8]) , }
};
}
