macro_rules! deps {
    () => {
        BuilderMethods!();
        TypeKind!();
    };
}

macro_rules! build_shift_expr_rhs {
    () => {
        deps!();
        # [doc = " Returns `rhs` sufficiently masked, truncated, and/or extended so that it can be used to shift"] # [doc = " `lhs`: it has the same size as `lhs`, and the value, when interpreted unsigned (no matter its"] # [doc = " type), will not exceed the size of `lhs`."] # [doc = ""] # [doc = " Shifts in MIR are all allowed to have mismatched LHS & RHS types, and signed RHS."] # [doc = " The shift methods in `BuilderMethods`, however, are fully homogeneous"] # [doc = " (both parameters and the return type are all the same size) and assume an unsigned RHS."] # [doc = ""] # [doc = " If `is_unchecked` is false, this masks the RHS to ensure it stays in-bounds,"] # [doc = " as the `BuilderMethods` shifts are UB for out-of-bounds shift amounts."] # [doc = " For 32- and 64-bit types, this matches the semantics"] # [doc = " of Java. (See related discussion on #1877 and #10183.)"] # [doc = ""] # [doc = " If `is_unchecked` is true, this does no masking, and adds sufficient `assume`"] # [doc = " calls or operation flags to preserve as much freedom to optimize as possible."] pub (crate) fn build_shift_expr_rhs < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , lhs : Bx :: Value , mut rhs : Bx :: Value , is_unchecked : bool ,) -> Bx :: Value { let mut rhs_llty = bx . cx () . val_ty (rhs) ; let mut lhs_llty = bx . cx () . val_ty (lhs) ; let mask = common :: shift_mask_val (bx , lhs_llty , rhs_llty , false) ; if ! is_unchecked { rhs = bx . and (rhs , mask) ; } if bx . cx () . type_kind (rhs_llty) == TypeKind :: Vector { rhs_llty = bx . cx () . element_type (rhs_llty) } if bx . cx () . type_kind (lhs_llty) == TypeKind :: Vector { lhs_llty = bx . cx () . element_type (lhs_llty) } let rhs_sz = bx . cx () . int_width (rhs_llty) ; let lhs_sz = bx . cx () . int_width (lhs_llty) ; if lhs_sz < rhs_sz { if is_unchecked { bx . unchecked_utrunc (rhs , lhs_llty) } else { bx . trunc (rhs , lhs_llty) } } else if lhs_sz > rhs_sz { assert ! (lhs_sz <= 256) ; bx . zext (rhs , lhs_llty) } else { rhs } }
    };
}

build_shift_expr_rhs!()