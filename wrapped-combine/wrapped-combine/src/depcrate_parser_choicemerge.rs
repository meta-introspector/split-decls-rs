// Generated macro for merge (macro)
macro_rules! Depcrate_parser_choicemerge {
() => {
// Module: crate::parser::choice
// Provides: {"merge"}
// Dependencies: {}
macro_rules ! merge { ($ head : ident) => { $ head . error } ; ($ head : ident $ ($ tail : ident) +) => { $ head . error . merge (merge ! ($ ($ tail) +)) } ; }
};
}
