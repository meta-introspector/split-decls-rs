macro_rules! deps {
    () => {
        SlotSize!();
        Builder!();
        AllowHigherAlign!();
        PassMode!();
        ForceRightAdjust!();
    };
}

macro_rules! emit_ptr_va_arg {
    () => {
        deps!();
        fn emit_ptr_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > , pass_mode : PassMode , slot_size : SlotSize , allow_higher_align : AllowHigherAlign , force_right_adjust : ForceRightAdjust ,) -> & 'll Value { let indirect = matches ! (pass_mode , PassMode :: Indirect) ; let allow_higher_align = matches ! (allow_higher_align , AllowHigherAlign :: Yes) ; let force_right_adjust = matches ! (force_right_adjust , ForceRightAdjust :: Yes) ; let slot_size = Align :: from_bytes (slot_size as u64) . unwrap () ; let layout = bx . cx . layout_of (target_ty) ; let (llty , size , align) = if indirect { (bx . cx . layout_of (Ty :: new_imm_ptr (bx . cx . tcx , target_ty)) . llvm_type (bx . cx) , bx . cx . data_layout () . pointer_size () , bx . cx . data_layout () . pointer_align () ,) } else { (layout . llvm_type (bx . cx) , layout . size , layout . align) } ; let (addr , addr_align) = emit_direct_ptr_va_arg (bx , list , size , align . abi , slot_size , allow_higher_align , force_right_adjust ,) ; if indirect { let tmp_ret = bx . load (llty , addr , addr_align) ; bx . load (bx . cx . layout_of (target_ty) . llvm_type (bx . cx) , tmp_ret , align . abi) } else { bx . load (llty , addr , addr_align) } }
    };
}

emit_ptr_va_arg!()