// Generated macro for llvm_err (function)
macro_rules! Depcrate_back_writellvm_err {
() => {
// Module: crate::back::write
// Provides: {"llvm_err"}
// Dependencies: {}
pub (crate) fn llvm_err < 'a > (dcx : DiagCtxtHandle < '_ > , err : LlvmError < 'a >) -> ! { match llvm :: last_error () { Some (llvm_err) => dcx . emit_fatal (WithLlvmError (err , llvm_err)) , None => dcx . emit_fatal (err) , } }
};
}
