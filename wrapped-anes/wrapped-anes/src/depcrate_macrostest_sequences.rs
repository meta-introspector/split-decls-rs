// Generated macro for test_sequences (macro)
macro_rules! Depcrate_macrostest_sequences {
() => {
// Module: crate::macros
// Provides: {"test_sequences"}
// Dependencies: {}
# [cfg (test)] macro_rules ! test_sequences { ($ ($ name : ident ($ ($ left : expr_2021 => $ right : expr_2021) ,* $ (,) ?)) ,* $ (,) ?) => { # [cfg (test)] mod tests { use super ::*; $ (# [test] fn $ name () { $ (assert_eq ! (& format ! ("{}" , $ left) , $ right) ;) * }) * } } }
};
}
