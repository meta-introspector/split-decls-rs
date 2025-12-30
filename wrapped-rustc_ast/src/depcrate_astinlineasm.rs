// Generated macro for InlineAsm (struct)
macro_rules! Depcrate_astInlineAsm {
() => {
// Module: crate::ast
// Provides: {"InlineAsm"}
// Dependencies: {}
# [doc = " Inline assembly."] # [doc = ""] # [doc = " E.g., `asm!(\"NOP\");`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct InlineAsm { pub asm_macro : AsmMacro , pub template : Vec < InlineAsmTemplatePiece > , pub template_strs : Box < [(Symbol , Option < Symbol > , Span)] > , pub operands : Vec < (InlineAsmOperand , Span) > , pub clobber_abis : Vec < (Symbol , Span) > , # [visitable (ignore)] pub options : InlineAsmOptions , pub line_spans : Vec < Span > , }
};
}
