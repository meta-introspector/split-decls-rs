// Generated macro for CInlineAsmOperand (enum)
macro_rules! Depcrate_inline_asmCInlineAsmOperand {
() => {
// Module: crate::inline_asm
// Provides: {"CInlineAsmOperand"}
// Dependencies: {}
pub (crate) enum CInlineAsmOperand < 'tcx > { In { reg : InlineAsmRegOrRegClass , value : Value , } , Out { reg : InlineAsmRegOrRegClass , late : bool , place : Option < CPlace < 'tcx > > , } , InOut { reg : InlineAsmRegOrRegClass , _late : bool , in_value : Value , out_place : Option < CPlace < 'tcx > > , } , Const { value : String , } , Symbol { symbol : String , } , }
};
}
