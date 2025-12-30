// Generated macro for macro_1296 (macro)
macro_rules! Depcrate_isa_x64_inst_argsmacro_1296 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"macro_1296"}
// Dependencies: {}
newtype_of_reg ! (Xmm , WritableXmm , OptionWritableXmm , reg_mem : (XmmMem , XmmMemAligned aligned : true) , reg_mem_imm : (XmmMemImm , XmmMemAlignedImm aligned : true) , Imm8Xmm , | reg | reg . class () == RegClass :: Float) ;
};
}
