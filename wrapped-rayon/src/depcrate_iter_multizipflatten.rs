// Generated macro for flatten (macro)
macro_rules! Depcrate_iter_multizipflatten {
() => {
// Module: crate::iter::multizip
// Provides: {"flatten"}
// Dependencies: {}
macro_rules ! flatten { ($ ($ T : ident) ,+) => { { # [allow (non_snake_case)] fn flatten <$ ($ T) ,+> (nest ! ($ ($ T) ,+) : nest ! ($ ($ T) ,+)) -> ($ ($ T ,) +) { ($ ($ T ,) +) } flatten } } ; }
};
}
