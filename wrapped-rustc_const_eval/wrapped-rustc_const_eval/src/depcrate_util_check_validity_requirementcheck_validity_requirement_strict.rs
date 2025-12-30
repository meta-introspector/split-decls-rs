// Generated macro for check_validity_requirement_strict (function)
macro_rules! Depcrate_util_check_validity_requirementcheck_validity_requirement_strict {
() => {
// Module: crate::util::check_validity_requirement
// Provides: {"check_validity_requirement_strict"}
// Dependencies: {}
# [doc = " Implements the 'strict' version of the [`check_validity_requirement`] checks; see that function"] # [doc = " for details."] fn check_validity_requirement_strict < 'tcx > (ty : TyAndLayout < 'tcx > , cx : & LayoutCx < 'tcx > , kind : ValidityRequirement ,) -> bool { let machine = CompileTimeMachine :: new (CanAccessMutGlobal :: No , CheckAlignment :: Error) ; let mut cx = InterpCx :: new (cx . tcx () , DUMMY_SP , cx . typing_env , machine) ; let allocated = cx . allocate (ty , MemoryKind :: Stack) . expect ("OOM: failed to allocate for uninit check") ; if kind == ValidityRequirement :: Zero { cx . write_bytes_ptr (allocated . ptr () , std :: iter :: repeat (0_u8) . take (ty . layout . size () . bytes_usize ()) ,) . expect ("failed to write bytes for zero valid check") ; } cx . validate_operand (& allocated . into () , false , false ,) . discard_err () . is_some () }
};
}
