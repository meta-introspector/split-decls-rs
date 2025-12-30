// Generated macro for impl_8963 (impl)
macro_rules! Depcrate_pointers_in_nomem_asm_blockimpl_8963 {
() => {
// Module: crate::pointers_in_nomem_asm_block
// Provides: {"impl_8963"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PointersInNomemAsmBlock { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: InlineAsm (asm) = & expr . kind { check_asm (cx , asm) ; } } }
};
}
