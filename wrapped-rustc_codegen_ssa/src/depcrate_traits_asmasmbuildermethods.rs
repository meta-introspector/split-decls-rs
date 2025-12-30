// Generated macro for AsmBuilderMethods (trait)
macro_rules! Depcrate_traits_asmAsmBuilderMethods {
() => {
// Module: crate::traits::asm
// Provides: {"AsmBuilderMethods"}
// Dependencies: {}
pub trait AsmBuilderMethods < 'tcx > : BackendTypes { # [doc = " Take an inline assembly expression and splat it out via LLVM"] fn codegen_inline_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [InlineAsmOperandRef < 'tcx , Self >] , options : InlineAsmOptions , line_spans : & [Span] , instance : Instance < '_ > , dest : Option < Self :: BasicBlock > , catch_funclet : Option < (Self :: BasicBlock , Option < & Self :: Funclet >) > ,) ; }
};
}
