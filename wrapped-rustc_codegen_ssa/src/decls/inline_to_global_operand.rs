macro_rules! deps {
    () => {
        GlobalAsmOperandRef!();
    };
}

macro_rules! inline_to_global_operand {
    () => {
        deps!();
        fn inline_to_global_operand < 'a , 'tcx , Cx : LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > > (cx : & 'a Cx , instance : Instance < 'tcx > , op : & InlineAsmOperand < 'tcx > ,) -> GlobalAsmOperandRef < 'tcx > { match op { InlineAsmOperand :: Const { value } => { let const_value = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . const_) ,) . eval (cx . tcx () , cx . typing_env () , value . span) . expect ("erroneous constant missed by mono item collection") ; let mono_type = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . ty ()) ,) ; let string = common :: asm_const_to_str (cx . tcx () , value . span , const_value , cx . layout_of (mono_type) ,) ; GlobalAsmOperandRef :: Const { string } } InlineAsmOperand :: SymFn { value } => { let mono_type = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . ty ()) ,) ; let instance = match mono_type . kind () { & ty :: FnDef (def_id , args) => { Instance :: expect_resolve (cx . tcx () , cx . typing_env () , def_id , args , value . span) } _ => bug ! ("asm sym is not a function") , } ; GlobalAsmOperandRef :: SymFn { instance } } InlineAsmOperand :: SymStatic { def_id } => { GlobalAsmOperandRef :: SymStatic { def_id : * def_id } } InlineAsmOperand :: In { .. } | InlineAsmOperand :: Out { .. } | InlineAsmOperand :: InOut { .. } | InlineAsmOperand :: Label { .. } => { bug ! ("invalid operand type for naked_asm!") } } }
    };
}

inline_to_global_operand!()