// Generated macro for impl_273 (impl)
macro_rules! Depcrate_hirimpl_273 {
() => {
// Module: crate::hir
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'hir > Block < 'hir > { pub fn innermost_block (& self) -> & Block < 'hir > { let mut block = self ; while let Some (Expr { kind : ExprKind :: Block (inner_block , _) , .. }) = block . expr { block = inner_block ; } block } }
};
}
