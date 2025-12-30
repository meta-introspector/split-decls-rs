// Generated macro for ConvertToAst (trait)
macro_rules! Depcrate_parserConvertToAst {
() => {
// Module: crate::parser
// Provides: {"ConvertToAst"}
// Dependencies: {}
# [doc = " Conversion trait with context."] # [doc = ""] # [doc = " Used to convert syn tokens into an AST, that we can then use to generate glue code. The context"] # [doc = " (`Ctx`) is used to pass in the attributes from the `#[wasm_bindgen]`, if needed."] pub (crate) trait ConvertToAst < Ctx > { # [doc = " What we are converting to."] type Target ; # [doc = " Convert into our target."] # [doc = ""] # [doc = " Since this is used in a procedural macro, use panic to fail."] fn convert (self , context : Ctx) -> Result < Self :: Target , Diagnostic > ; }
};
}
