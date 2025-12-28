macro_rules! deps {
    () => {
        AllowHigherAlign!();
        SlotSize!();
        ForceRightAdjust!();
        Builder!();
        PassMode!();
        IntPredicate!();
    };
}

macro_rules! emit_aapcs_va_arg {
    () => {
        deps!();
        fn emit_aapcs_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let va_list_addr = list . immediate () ; let ptr_offset = 8 ; let i32_offset = 4 ; let gr_top = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (ptr_offset)) ; let vr_top = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (2 * ptr_offset)) ; let gr_offs = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (3 * ptr_offset)) ; let vr_offs = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (3 * ptr_offset + i32_offset)) ; let layout = bx . cx . layout_of (target_ty) ; let maybe_reg = bx . append_sibling_block ("va_arg.maybe_reg") ; let in_reg = bx . append_sibling_block ("va_arg.in_reg") ; let on_stack = bx . append_sibling_block ("va_arg.on_stack") ; let end = bx . append_sibling_block ("va_arg.end") ; let zero = bx . const_i32 (0) ; let offset_align = Align :: from_bytes (4) . unwrap () ; let gr_type = target_ty . is_any_ptr () || target_ty . is_integral () ; let (reg_off , reg_top , slot_size) = if gr_type { let nreg = layout . size . bytes () . div_ceil (8) ; (gr_offs , gr_top , nreg * 8) } else { let nreg = layout . size . bytes () . div_ceil (16) ; (vr_offs , vr_top , nreg * 16) } ; let mut reg_off_v = bx . load (bx . type_i32 () , reg_off , offset_align) ; let use_stack = bx . icmp (IntPredicate :: IntSGE , reg_off_v , zero) ; bx . cond_br (use_stack , on_stack , maybe_reg) ; bx . switch_to_block (maybe_reg) ; if gr_type && layout . align . abi . bytes () > 8 { reg_off_v = bx . add (reg_off_v , bx . const_i32 (15)) ; reg_off_v = bx . and (reg_off_v , bx . const_i32 (- 16)) ; } let new_reg_off_v = bx . add (reg_off_v , bx . const_i32 (slot_size as i32)) ; bx . store (new_reg_off_v , reg_off , offset_align) ; let use_stack = bx . icmp (IntPredicate :: IntSGT , new_reg_off_v , zero) ; bx . cond_br (use_stack , on_stack , in_reg) ; bx . switch_to_block (in_reg) ; let top_type = bx . type_ptr () ; let top = bx . load (top_type , reg_top , dl . pointer_align () . abi) ; let mut reg_addr = bx . ptradd (top , reg_off_v) ; if bx . tcx () . sess . target . endian == Endian :: Big && layout . size . bytes () != slot_size { let offset = bx . const_i32 ((slot_size - layout . size . bytes ()) as i32) ; reg_addr = bx . ptradd (reg_addr , offset) ; } let reg_type = layout . llvm_type (bx) ; let reg_value = bx . load (reg_type , reg_addr , layout . align . abi) ; bx . br (end) ; bx . switch_to_block (on_stack) ; let stack_value = emit_ptr_va_arg (bx , list , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , AllowHigherAlign :: Yes , ForceRightAdjust :: No ,) ; bx . br (end) ; bx . switch_to_block (end) ; let val = bx . phi (layout . immediate_llvm_type (bx) , & [reg_value , stack_value] , & [in_reg , on_stack]) ; val }
    };
}

emit_aapcs_va_arg!()