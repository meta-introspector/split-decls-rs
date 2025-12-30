// Generated macro for parser (module)
macro_rules! Depcrate_astparser {
() => {
// Module: crate::ast
// Provides: {"parser"}
// Dependencies: {}
mod parser { use pest_derive :: Parser ; # [derive (Parser)] # [grammar = "parser/dot.pest"] pub struct DotParser ; }
};
}
