// Generated macro for concatln (macro)
macro_rules! Depcrate_common_gen_rustconcatln {
() => {
// Module: crate::common::gen_rust
// Provides: {"concatln"}
// Dependencies: {}
macro_rules ! concatln { ($ ($ lines : expr) ,* $ (,) ?) => { concat ! ($ ($ lines , "\n") ,*) } ; }
};
}
