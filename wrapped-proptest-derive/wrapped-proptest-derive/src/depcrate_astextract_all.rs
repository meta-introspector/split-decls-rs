// Generated macro for extract_all (function)
macro_rules! Depcrate_astextract_all {
() => {
// Module: crate::ast
// Provides: {"extract_all"}
// Dependencies: {}
# [doc = " Wraps the given strategy producing expression with a move into"] # [doc = " `params_<to>` from `FromReg`. This is used when the given `c` expects"] # [doc = " `params_<to>` to be there."] pub fn extract_all (c : Ctor , to : usize , from : FromReg) -> Ctor { extract (c , ToReg :: Range (to) , from) }
};
}
