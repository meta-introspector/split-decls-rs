// Generated macro for bail (macro)
macro_rules! Depcrate_ir_pccbail {
() => {
// Module: crate::ir::pcc
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ err : tt) => { { return Err (PccError ::$ err) ; } } ; }
};
}
