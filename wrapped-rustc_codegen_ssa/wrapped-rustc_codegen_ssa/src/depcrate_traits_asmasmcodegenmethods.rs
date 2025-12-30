// Generated macro for AsmCodegenMethods (trait)
macro_rules! Depcrate_traits_asmAsmCodegenMethods {
() => {
// Module: crate::traits::asm
// Provides: {"AsmCodegenMethods"}
// Dependencies: {}
pub trait AsmCodegenMethods < 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , line_spans : & [Span] ,) ; # [doc = " The mangled name of this instance"] # [doc = ""] # [doc = " Additional mangling is used on"] # [doc = " some targets to add a leading underscore (Mach-O)"] # [doc = " or byte count suffixes (x86 Windows)."] fn mangled_name (& self , instance : Instance < 'tcx >) -> String ; }
};
}
