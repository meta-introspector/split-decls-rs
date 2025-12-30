// Generated macro for extract_api (function)
macro_rules! Depcrate_astextract_api {
() => {
// Module: crate::ast
// Provides: {"extract_api"}
// Dependencies: {}
# [doc = " Wraps the given strategy producing expression with a move into `params`"] # [doc = " (literally named like that) from `FromReg`. This is used when the given"] # [doc = " `c` expects `params` to be there."] pub fn extract_api (c : Ctor , from : FromReg) -> Ctor { extract (c , ToReg :: API , from) }
};
}
