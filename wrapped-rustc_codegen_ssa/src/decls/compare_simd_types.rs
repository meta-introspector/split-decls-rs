macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! compare_simd_types {
    () => {
        deps!();
        pub fn compare_simd_types < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , lhs : Bx :: Value , rhs : Bx :: Value , t : Ty < 'tcx > , ret_ty : Bx :: Type , op : BinOp ,) -> Bx :: Value { let signed = match t . kind () { ty :: Float (_) => { let cmp = bin_op_to_fcmp_predicate (op) ; let cmp = bx . fcmp (cmp , lhs , rhs) ; return bx . sext (cmp , ret_ty) ; } ty :: Uint (_) => false , ty :: Int (_) => true , _ => bug ! ("compare_simd_types: invalid SIMD type") , } ; let cmp = bin_op_to_icmp_predicate (op , signed) ; let cmp = bx . icmp (cmp , lhs , rhs) ; bx . sext (cmp , ret_ty) }
    };
}

compare_simd_types!();