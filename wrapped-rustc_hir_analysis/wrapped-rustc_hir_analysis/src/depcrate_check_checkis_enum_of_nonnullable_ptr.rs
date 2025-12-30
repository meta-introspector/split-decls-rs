// Generated macro for is_enum_of_nonnullable_ptr (function)
macro_rules! Depcrate_check_checkis_enum_of_nonnullable_ptr {
() => {
// Module: crate::check::check
// Provides: {"is_enum_of_nonnullable_ptr"}
// Dependencies: {}
fn is_enum_of_nonnullable_ptr < 'tcx > (tcx : TyCtxt < 'tcx > , adt_def : AdtDef < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> bool { if adt_def . repr () . inhibit_enum_layout_opt () { return false ; } let [var_one , var_two] = & adt_def . variants () . raw [..] else { return false ; } ; let (([] , [field]) | ([field] , [])) = (& var_one . fields . raw [..] , & var_two . fields . raw [..]) else { return false ; } ; matches ! (field . ty (tcx , args) . kind () , ty :: FnPtr (..) | ty :: Ref (..)) }
};
}
