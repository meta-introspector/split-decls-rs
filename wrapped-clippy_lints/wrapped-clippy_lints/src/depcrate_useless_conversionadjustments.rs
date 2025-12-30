// Generated macro for adjustments (function)
macro_rules! Depcrate_useless_conversionadjustments {
() => {
// Module: crate::useless_conversion
// Provides: {"adjustments"}
// Dependencies: {}
fn adjustments (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> String { let mut prefix = String :: new () ; for adj in cx . typeck_results () . expr_adjustments (expr) { match adj . kind { Adjust :: Deref (_) => prefix = format ! ("*{prefix}") , Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: Mut { .. })) => prefix = format ! ("&mut {prefix}") , Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: Not)) => prefix = format ! ("&{prefix}") , _ => { } , } } prefix }
};
}
