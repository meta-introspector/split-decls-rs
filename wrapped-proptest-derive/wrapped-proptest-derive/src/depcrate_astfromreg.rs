// Generated macro for FromReg (enum)
macro_rules! Depcrate_astFromReg {
() => {
// Module: crate::ast
// Provides: {"FromReg"}
// Dependencies: {}
# [doc = " The right hand side (RHS) of a let binding of parameters."] pub enum FromReg { # [doc = " Denotes a move from the top parameter given in the arguments of"] # [doc = " `arbitrary_with`."] Top , # [doc = " Denotes a move from a variable `params_<x>` where `<x>` is the given"] # [doc = " number."] Num (usize) , }
};
}
