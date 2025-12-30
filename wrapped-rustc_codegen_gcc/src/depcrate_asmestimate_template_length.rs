// Generated macro for estimate_template_length (function)
macro_rules! Depcrate_asmestimate_template_length {
() => {
// Module: crate::asm
// Provides: {"estimate_template_length"}
// Dependencies: {}
fn estimate_template_length (template : & [InlineAsmTemplatePiece] , constants_len : usize , att_dialect : bool ,) -> usize { let len : usize = template . iter () . map (| piece | { match * piece { InlineAsmTemplatePiece :: String (ref string) => string . len () , InlineAsmTemplatePiece :: Placeholder { .. } => { 3 } } }) . sum () ; let mut res = (len as f32 * 1.05) as usize + constants_len ; if att_dialect { res += INTEL_SYNTAX_INS . len () + ATT_SYNTAX_INS . len () ; } res }
};
}
