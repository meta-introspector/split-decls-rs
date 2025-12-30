// Generated macro for generate_patterns (function)
macro_rules! Depcrate_generatorgenerate_patterns {
() => {
// Module: crate::generator
// Provides: {"generate_patterns"}
// Dependencies: {}
fn generate_patterns (rules : & [OptimizedRule] , uses_eoi : bool) -> TokenStream { let mut rules : Vec < TokenStream > = rules . iter () . map (| rule | { let rule = format_ident ! ("r#{}" , rule . name) ; quote ! { Rule ::# rule => rules ::# rule (state) } }) . collect () ; if uses_eoi { rules . push (quote ! { Rule :: EOI => rules :: EOI (state) }) ; } quote ! { # (# rules) ,* } }
};
}
