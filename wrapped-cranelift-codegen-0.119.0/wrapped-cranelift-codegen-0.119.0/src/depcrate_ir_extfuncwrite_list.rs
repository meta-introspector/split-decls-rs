// Generated macro for write_list (function)
macro_rules! Depcrate_ir_extfuncwrite_list {
() => {
// Module: crate::ir::extfunc
// Provides: {"write_list"}
// Dependencies: {}
fn write_list (f : & mut fmt :: Formatter , args : & [AbiParam]) -> fmt :: Result { match args . split_first () { None => { } Some ((first , rest)) => { write ! (f , "{first}") ? ; for arg in rest { write ! (f , ", {arg}") ? ; } } } Ok (()) }
};
}
