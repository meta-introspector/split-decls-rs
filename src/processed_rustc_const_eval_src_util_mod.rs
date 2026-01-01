// SRC: ../rust/compiler/rustc_const_eval/src/util/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */
use crate::rustc_complete::mir;


pub use self::alignment::{is_disaligned, is_within_packed};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
pub use self::check_validity_requirement::check_validity_requirement;
pub(crate) use self::check_validity_requirement::validate_scalar_in_layout;
pub use self::compare_types::{relate_types, sub_types};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=binop_left_homogeneous | COMPLEXITY=8 | LINES=15 */
pub use self::type_name::type_name;

/// Classify whether an operator is "left-homogeneous", i.e., the LHS has the
/// same type as the result.
#[inline]
pub fn binop_left_homogeneous(op: mir::BinOp) -> bool {
    use crate::rustc_complete::mir::BinOp::*;
    match op {
        Add | AddUnchecked | Sub | SubUnchecked | Mul | MulUnchecked | Div | Rem | BitXor
        | BitAnd | BitOr | Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => true,
        AddWithOverflow | SubWithOverflow | MulWithOverflow | Eq | Ne | Lt | Le | Gt | Ge | Cmp => {
            false
        }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=binop_right_homogeneous | COMPLEXITY=7 | LINES=13 */

/// Classify whether an operator is "right-homogeneous", i.e., the RHS has the
/// same type as the LHS.
#[inline]
pub fn binop_right_homogeneous(op: mir::BinOp) -> bool {
    use crate::rustc_complete::mir::BinOp::*;
    match op {
        Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul
        | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Eq | Ne | Lt
        | Le | Gt | Ge | Cmp => true,
        Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => false,
    }
}