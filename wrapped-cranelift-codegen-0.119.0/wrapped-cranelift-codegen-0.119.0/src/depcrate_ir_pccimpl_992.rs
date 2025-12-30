// Generated macro for impl_992 (impl)
macro_rules! Depcrate_ir_pccimpl_992 {
() => {
// Module: crate::ir::pcc
// Provides: {"impl_992"}
// Dependencies: {}
impl BaseExpr { # [doc = " Is one base less than or equal to another? (We can't always"] # [doc = " know; in such cases, returns `false`.)"] fn le (lhs : & BaseExpr , rhs : & BaseExpr) -> bool { lhs == rhs || * lhs == BaseExpr :: None || * rhs == BaseExpr :: Max } # [doc = " Compute some BaseExpr that will be less than or equal to both"] # [doc = " inputs. This is a generalization of `min` (but looser)."] fn min (lhs : & BaseExpr , rhs : & BaseExpr) -> BaseExpr { if lhs == rhs { lhs . clone () } else if * lhs == BaseExpr :: Max { rhs . clone () } else if * rhs == BaseExpr :: Max { lhs . clone () } else { BaseExpr :: None } } # [doc = " Compute some BaseExpr that will be greater than or equal to"] # [doc = " both inputs."] fn max (lhs : & BaseExpr , rhs : & BaseExpr) -> BaseExpr { if lhs == rhs { lhs . clone () } else if * lhs == BaseExpr :: None { rhs . clone () } else if * rhs == BaseExpr :: None { lhs . clone () } else { BaseExpr :: Max } } }
};
}
