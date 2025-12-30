// Generated macro for parse_args (function)
macro_rules! Depcrate_asmparse_args {
() => {
// Module: crate::asm
// Provides: {"parse_args"}
// Dependencies: {}
fn parse_args < 'a > (ecx : & ExtCtxt < 'a > , sp : Span , tts : TokenStream , asm_macro : AsmMacro ,) -> PResult < 'a , ValidatedAsmArgs > { let args = parse_asm_args (& mut ecx . new_parser_from_tts (tts) , sp , asm_macro) ? ; validate_asm_args (ecx , asm_macro , args) }
};
}
