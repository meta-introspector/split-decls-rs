macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! x86_64_sysv64_va_arg_from_memory {
    () => {
        deps!();
        fn x86_64_sysv64_va_arg_from_memory < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , va_list_addr : & 'll Value , layout : TyAndLayout < 'tcx , Ty < 'tcx > > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let ptr_align_abi = dl . data_layout () . pointer_align () . abi ; let overflow_arg_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . const_usize (8)) ; let overflow_arg_area_v = bx . load (bx . type_ptr () , overflow_arg_area_ptr , ptr_align_abi) ; if layout . layout . align . abi . bytes () > 8 { unreachable ! ("all instances of VaArgSafe have an alignment <= 8") ; } let mem_addr = overflow_arg_area_v ; let size_in_bytes = layout . layout . size () . bytes () ; let offset = bx . const_i32 (size_in_bytes . next_multiple_of (8) as i32) ; let overflow_arg_area = bx . inbounds_ptradd (overflow_arg_area_v , offset) ; bx . store (overflow_arg_area , overflow_arg_area_ptr , ptr_align_abi) ; mem_addr }
    };
}

x86_64_sysv64_va_arg_from_memory!();