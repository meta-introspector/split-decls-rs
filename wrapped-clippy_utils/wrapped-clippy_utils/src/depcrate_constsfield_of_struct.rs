// Generated macro for field_of_struct (function)
macro_rules! Depcrate_constsfield_of_struct {
() => {
// Module: crate::consts
// Provides: {"field_of_struct"}
// Dependencies: {}
fn field_of_struct < 'tcx > (adt_def : ty :: AdtDef < 'tcx > , tcx : TyCtxt < 'tcx > , value : ConstValue , ty : Ty < 'tcx > , field : Symbol ,) -> Option < (ConstValue , Ty < 'tcx >) > { if let Some (dc) = tcx . try_destructure_mir_constant_for_user_output (value , ty) && let Some (dc_variant) = dc . variant && let Some (variant) = adt_def . variants () . get (dc_variant) && let Some (field_idx) = variant . fields . iter () . position (| el | el . name == field) { dc . fields . get (field_idx) . copied () } else { None } }
};
}
