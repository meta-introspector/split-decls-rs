// Generated macro for InlineAsmOperandRef (enum)
macro_rules! Depcrate_traits_asmInlineAsmOperandRef {
() => {
// Module: crate::traits::asm
// Provides: {"InlineAsmOperandRef"}
// Dependencies: {}
# [derive (Debug)] pub enum InlineAsmOperandRef < 'tcx , B : BackendTypes + ? Sized > { In { reg : InlineAsmRegOrRegClass , value : OperandRef < 'tcx , B :: Value > , } , Out { reg : InlineAsmRegOrRegClass , late : bool , place : Option < PlaceRef < 'tcx , B :: Value > > , } , InOut { reg : InlineAsmRegOrRegClass , late : bool , in_value : OperandRef < 'tcx , B :: Value > , out_place : Option < PlaceRef < 'tcx , B :: Value > > , } , Const { string : String , } , SymFn { instance : Instance < 'tcx > , } , SymStatic { def_id : DefId , } , Label { label : B :: BasicBlock , } , }
};
}
