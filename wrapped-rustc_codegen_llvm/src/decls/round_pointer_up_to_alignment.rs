macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! round_pointer_up_to_alignment {
    () => {
        deps!();
        fn round_pointer_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , addr : & 'll Value , align : Align , ptr_ty : & 'll Type ,) -> & 'll Value { let ptr = bx . inbounds_ptradd (addr , bx . const_i32 (align . bytes () as i32 - 1)) ; bx . call_intrinsic ("llvm.ptrmask" , & [ptr_ty , bx . type_i32 ()] , & [ptr , bx . const_int (bx . isize_ty , - (align . bytes () as isize) as i64)] ,) }
    };
}

round_pointer_up_to_alignment!();