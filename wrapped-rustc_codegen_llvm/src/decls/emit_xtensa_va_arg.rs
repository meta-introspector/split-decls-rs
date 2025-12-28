macro_rules! deps {
    () => {
        IntPredicate!();
        Builder!();
    };
}

macro_rules! emit_xtensa_va_arg {
    () => {
        deps!();
        fn emit_xtensa_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let va_list_addr = list . immediate () ; let layout = bx . cx . layout_of (target_ty) ; let from_stack = bx . append_sibling_block ("va_arg.from_stack") ; let from_regsave = bx . append_sibling_block ("va_arg.from_regsave") ; let end = bx . append_sibling_block ("va_arg.end") ; let ptr_align_abi = bx . tcx () . data_layout . pointer_align () . abi ; let va_reg_offset = 4 ; let va_ndx_offset = va_reg_offset + 4 ; let offset_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (va_ndx_offset)) ; let offset = bx . load (bx . type_i32 () , offset_ptr , bx . tcx () . data_layout . i32_align . abi) ; let offset = round_up_to_alignment (bx , offset , layout . align . abi) ; let slot_size = layout . size . align_to (Align :: from_bytes (4) . unwrap ()) . bytes () as i32 ; let offset_next = bx . add (offset , bx . const_i32 (slot_size)) ; let regsave_size = bx . const_i32 (24) ; let use_regsave = bx . icmp (IntPredicate :: IntULE , offset_next , regsave_size) ; bx . cond_br (use_regsave , from_regsave , from_stack) ; bx . switch_to_block (from_regsave) ; bx . store (offset_next , offset_ptr , ptr_align_abi) ; let regsave_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (va_reg_offset)) ; let regsave_area = bx . load (bx . type_ptr () , regsave_area_ptr , ptr_align_abi) ; let regsave_value_ptr = bx . inbounds_ptradd (regsave_area , offset) ; bx . br (end) ; bx . switch_to_block (from_stack) ; let stack_offset_start = bx . const_i32 (32) ; let needs_correction = bx . icmp (IntPredicate :: IntULE , offset , stack_offset_start) ; let offset_corrected = bx . select (needs_correction , stack_offset_start , offset) ; let offset_next_corrected = bx . add (offset_next , bx . const_i32 (slot_size)) ; bx . store (offset_next_corrected , offset_ptr , ptr_align_abi) ; let stack_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (0)) ; let stack_area = bx . load (bx . type_ptr () , stack_area_ptr , ptr_align_abi) ; let stack_value_ptr = bx . inbounds_ptradd (stack_area , offset_corrected) ; bx . br (end) ; bx . switch_to_block (end) ; assert ! (bx . tcx () . sess . target . endian == Endian :: Little) ; let value_ptr = bx . phi (bx . type_ptr () , & [regsave_value_ptr , stack_value_ptr] , & [from_regsave , from_stack]) ; return bx . load (layout . llvm_type (bx) , value_ptr , layout . align . abi) ; }
    };
}

emit_xtensa_va_arg!()