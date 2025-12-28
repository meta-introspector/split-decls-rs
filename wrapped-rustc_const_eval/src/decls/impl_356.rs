macro_rules! deps {
    () => {
        PathElem!();
        CtfeValidationMode!();
        MPlaceTy!();
        InterpCx!();
        RefTracking!();
        PlaceTy!();
        ValidityVisitor!();
        RangeSet!();
        Machine!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { fn validate_operand_internal (& mut self , val : & PlaceTy < 'tcx , M :: Provenance > , path : Vec < PathElem > , ref_tracking : Option < & mut RefTracking < MPlaceTy < 'tcx , M :: Provenance > , Vec < PathElem > > > , ctfe_mode : Option < CtfeValidationMode > , reset_provenance_and_padding : bool ,) -> InterpResult < 'tcx > { trace ! ("validate_operand_internal: {:?}, {:?}" , * val , val . layout . ty) ; self . run_for_validation_mut (| ecx | { let reset_padding = reset_provenance_and_padding && { ecx . place_to_op (val) ? . as_mplace_or_imm () . is_left () } ; let mut v = ValidityVisitor { path , ref_tracking , ctfe_mode , ecx , reset_provenance_and_padding , data_bytes : reset_padding . then_some (RangeSet (Vec :: new ())) , } ; v . visit_value (val) ? ; v . reset_padding (val) ? ; interp_ok (()) }) . map_err_info (| err | { if ! matches ! (err . kind () , err_ub ! (ValidationError { .. }) | InterpErrorKind :: InvalidProgram (_) | InterpErrorKind :: Unsupported (UnsupportedOpInfo :: ExternTypeField)) { bug ! ("Unexpected error during validation: {}" , format_interp_error (self . tcx . dcx () , err)) ; } err }) } # [doc = " This function checks the data at `val` to be const-valid."] # [doc = " `val` is assumed to cover valid memory if it is an indirect operand."] # [doc = " It will error if the bits at the destination do not match the ones described by the layout."] # [doc = ""] # [doc = " `ref_tracking` is used to record references that we encounter so that they"] # [doc = " can be checked recursively by an outside driving loop."] # [doc = ""] # [doc = " `constant` controls whether this must satisfy the rules for constants:"] # [doc = " - no pointers to statics."] # [doc = " - no `UnsafeCell` or non-ZST `&mut`."] # [inline (always)] pub (crate) fn const_validate_operand (& mut self , val : & PlaceTy < 'tcx , M :: Provenance > , path : Vec < PathElem > , ref_tracking : & mut RefTracking < MPlaceTy < 'tcx , M :: Provenance > , Vec < PathElem > > , ctfe_mode : CtfeValidationMode ,) -> InterpResult < 'tcx > { self . validate_operand_internal (val , path , Some (ref_tracking) , Some (ctfe_mode) , false ,) } # [doc = " This function checks the data at `val` to be runtime-valid."] # [doc = " `val` is assumed to cover valid memory if it is an indirect operand."] # [doc = " It will error if the bits at the destination do not match the ones described by the layout."] # [inline (always)] pub fn validate_operand (& mut self , val : & PlaceTy < 'tcx , M :: Provenance > , recursive : bool , reset_provenance_and_padding : bool ,) -> InterpResult < 'tcx > { let _trace = enter_trace_span ! (M , "validate_operand" , recursive , reset_provenance_and_padding , ? val ,) ; if ! recursive { return self . validate_operand_internal (val , vec ! [] , None , None , reset_provenance_and_padding ,) ; } let mut ref_tracking = RefTracking :: empty () ; self . validate_operand_internal (val , vec ! [] , Some (& mut ref_tracking) , None , reset_provenance_and_padding ,) ? ; while let Some ((mplace , path)) = ref_tracking . todo . pop () { self . validate_operand_internal (& mplace . into () , path , Some (& mut ref_tracking) , None , false ,) ? ; } interp_ok (()) } }
    };
}

impl_356!();