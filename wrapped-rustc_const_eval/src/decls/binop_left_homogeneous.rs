macro_rules! binop_left_homogeneous {
    () => {
        # [doc = " Classify whether an operator is \"left-homogeneous\", i.e., the LHS has the"] # [doc = " same type as the result."] # [inline] pub fn binop_left_homogeneous (op : mir :: BinOp) -> bool { use rustc_middle :: mir :: BinOp :: * ; match op { Add | AddUnchecked | Sub | SubUnchecked | Mul | MulUnchecked | Div | Rem | BitXor | BitAnd | BitOr | Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => true , AddWithOverflow | SubWithOverflow | MulWithOverflow | Eq | Ne | Lt | Le | Gt | Ge | Cmp => { false } } }
    };
}

binop_left_homogeneous!();