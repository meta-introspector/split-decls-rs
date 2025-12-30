// Generated macro for error (macro)
macro_rules! Depcrate_errorerror {
() => {
// Module: crate::error
// Provides: {"error"}
// Dependencies: {}
# [doc = " A macro constructing fatal errors that do not halt compilation immediately."] macro_rules ! error { ($ error : ident , $ code : ident , $ msg : expr) => { pub fn $ error (ctx : Ctx) { ctx . error (mk_err_msg ! ($ code , $ msg)) } } ; ($ error : ident ($ ($ arg : ident : $ arg_ty : ty) ,*) , $ code : ident , $ msg : expr , $ ($ fmt : tt) +) => { pub fn $ error (ctx : Ctx , $ ($ arg : $ arg_ty) ,*) { ctx . error (mk_err_msg ! ($ code , format ! ($ msg , $ ($ fmt) +))) } } ; }
};
}
