macro_rules! deps {
    () => {
        CompileTimeMachine!();
        InterpCx!();
        CtfeValidationMode!();
        MPlaceTy!();
        RefTracking!();
    };
}

macro_rules! const_validate_mplace {
    () => {
        deps!();
        # [inline (always)] fn const_validate_mplace < 'tcx > (ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , mplace : & MPlaceTy < 'tcx > , cid : GlobalId < 'tcx > ,) -> Result < () , ErrorHandled > { let alloc_id = mplace . ptr () . provenance . unwrap () . alloc_id () ; let mut ref_tracking = RefTracking :: new (mplace . clone ()) ; let mut inner = false ; while let Some ((mplace , path)) = ref_tracking . next () { let mode = match ecx . tcx . static_mutability (cid . instance . def_id ()) { _ if cid . promoted . is_some () => CtfeValidationMode :: Promoted , Some (mutbl) => CtfeValidationMode :: Static { mutbl } , None => { CtfeValidationMode :: Const { allow_immutable_unsafe_cell : ! inner } } } ; ecx . const_validate_operand (& mplace . into () , path , & mut ref_tracking , mode) . report_err () . map_err (| error | report_validation_error (& ecx , cid , error , alloc_id)) ? ; inner = true ; } Ok (()) }
    };
}

const_validate_mplace!();