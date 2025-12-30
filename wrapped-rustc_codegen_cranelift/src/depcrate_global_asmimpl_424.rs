// Generated macro for impl_424 (impl)
macro_rules! Depcrate_global_asmimpl_424 {
() => {
// Module: crate::global_asm
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'tcx > AsmCodegenMethods < 'tcx > for GlobalAsmContext < '_ , 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , _line_spans : & [Span] ,) { codegen_global_asm_inner (self . tcx , self . global_asm , template , operands , options) ; } fn mangled_name (& self , instance : Instance < 'tcx >) -> String { let symbol_name = self . tcx . symbol_name (instance) . name . to_owned () ; if self . tcx . sess . target . is_like_darwin { format ! ("_{symbol_name}") } else { symbol_name } } }
};
}
