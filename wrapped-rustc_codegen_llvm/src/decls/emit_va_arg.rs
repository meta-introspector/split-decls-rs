macro_rules! deps {
    () => {
        Builder!();
        ForceRightAdjust!();
        AllowHigherAlign!();
        SlotSize!();
        PassMode!();
    };
}

macro_rules! emit_va_arg {
    () => {
        deps!();
        pub (super) fn emit_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , addr : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let target = & bx . cx . tcx . sess . target ; match & * target . arch { "x86" => emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes4 , if target . is_like_windows { AllowHigherAlign :: No } else { AllowHigherAlign :: Yes } , ForceRightAdjust :: No ,) , "aarch64" | "arm64ec" if target . is_like_windows || target . is_like_darwin => { emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , if target . is_like_windows { AllowHigherAlign :: No } else { AllowHigherAlign :: Yes } , ForceRightAdjust :: No ,) } "aarch64" => emit_aapcs_va_arg (bx , addr , target_ty) , "arm" => { assert ! (bx . cx . size_of (target_ty) . bytes () <= 16) ; emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes4 , AllowHigherAlign :: Yes , ForceRightAdjust :: No ,) } "s390x" => emit_s390x_va_arg (bx , addr , target_ty) , "powerpc" => emit_powerpc_va_arg (bx , addr , target_ty) , "powerpc64" | "powerpc64le" => emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , AllowHigherAlign :: Yes , match & * target . arch { "powerpc64" => ForceRightAdjust :: Yes , _ => ForceRightAdjust :: No , } ,) , "x86_64" if target . is_like_windows => { let target_ty_size = bx . cx . size_of (target_ty) . bytes () ; emit_ptr_va_arg (bx , addr , target_ty , if target_ty_size > 8 || ! target_ty_size . is_power_of_two () { PassMode :: Indirect } else { PassMode :: Direct } , SlotSize :: Bytes8 , AllowHigherAlign :: No , ForceRightAdjust :: No ,) } "x86_64" => emit_x86_64_sysv64_va_arg (bx , addr , target_ty) , "xtensa" => emit_xtensa_va_arg (bx , addr , target_ty) , _ => bx . va_arg (addr . immediate () , bx . cx . layout_of (target_ty) . llvm_type (bx . cx)) , } }
    };
}

emit_va_arg!()