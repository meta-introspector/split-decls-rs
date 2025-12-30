// Generated macro for go_match (macro)
macro_rules! Depcrate_tokenizergo_match {
() => {
// Module: crate::tokenizer
// Provides: {"go_match"}
// Dependencies: {}
macro_rules ! go_match (($ me : ident : $ x : expr , $ ($ pats : pat) ,+ => $ ($ cmds : tt) *) => (match $ x { $ ($ pats) |+ => go ! ($ me : $ ($ cmds) *) , _ => () , })) ;
};
}
