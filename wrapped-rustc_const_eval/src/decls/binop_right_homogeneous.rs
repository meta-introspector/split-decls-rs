macro_rules! binop_right_homogeneous {
    () => {
        # [doc = " Classify whether an operator is \"right-homogeneous\", i.e., the RHS has the"] # [doc = " same type as the LHS."] # [inline] pub fn binop_right_homogeneous (op : mir :: BinOp) -> bool { use rustc_middle :: mir :: BinOp :: * ; match op { Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Eq | Ne | Lt | Le | Gt | Ge | Cmp => true , Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => false , } }
    };
}

binop_right_homogeneous!();