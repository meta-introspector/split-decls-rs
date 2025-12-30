// Generated macro for ensure (macro)
macro_rules! Depcrate_ir_pccensure {
() => {
// Module: crate::ir::pcc
// Provides: {"ensure"}
// Dependencies: {}
macro_rules ! ensure { ($ condition : expr , $ err : tt $ (,) ?) => { if !$ condition { return Err (PccError ::$ err) ; } } ; }
};
}
