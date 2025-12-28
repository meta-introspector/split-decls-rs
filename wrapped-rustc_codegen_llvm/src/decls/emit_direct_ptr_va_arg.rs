macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! emit_direct_ptr_va_arg {
    () => {
        deps!();
        fn emit_direct_ptr_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , size : Size , align : Align , slot_size : Align , allow_higher_align : bool , force_right_adjust : bool ,) -> (& 'll Value , Align) { let va_list_ty = bx . type_ptr () ; let va_list_addr = list . immediate () ; let ptr_align_abi = bx . tcx () . data_layout . pointer_align () . abi ; let ptr = bx . load (va_list_ty , va_list_addr , ptr_align_abi) ; let (addr , addr_align) = if allow_higher_align && align > slot_size { (round_pointer_up_to_alignment (bx , ptr , align , bx . type_ptr ()) , align) } else { (ptr , slot_size) } ; let aligned_size = size . align_to (slot_size) . bytes () as i32 ; let full_direct_size = bx . cx () . const_i32 (aligned_size) ; let next = bx . inbounds_ptradd (addr , full_direct_size) ; bx . store (next , va_list_addr , ptr_align_abi) ; if size . bytes () < slot_size . bytes () && bx . tcx () . sess . target . endian == Endian :: Big && force_right_adjust { let adjusted_size = bx . cx () . const_i32 ((slot_size . bytes () - size . bytes ()) as i32) ; let adjusted = bx . inbounds_ptradd (addr , adjusted_size) ; (adjusted , addr_align) } else { (addr , addr_align) } }
    };
}

emit_direct_ptr_va_arg!()