// Generated macro for ToReg (enum)
macro_rules! Depcrate_astToReg {
() => {
// Module: crate::ast
// Provides: {"ToReg"}
// Dependencies: {}
# [doc = " The left hand side (LHS) of a let binding of parameters."] pub enum ToReg { # [doc = " Denotes a move and declaration to a sequence of variables from"] # [doc = " `params_0` to `params_x`."] Range (usize) , # [doc = " Denotes a move and declaration of a special variable `params` that is"] # [doc = " user facing and is ALWAYS named `params`."] # [doc = ""] # [doc = " To change the name this linearises to is considered a breaking change"] # [doc = " wrt. semver."] API , }
};
}
