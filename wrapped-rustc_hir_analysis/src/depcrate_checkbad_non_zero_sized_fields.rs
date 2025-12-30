// Generated macro for bad_non_zero_sized_fields (function)
macro_rules! Depcrate_checkbad_non_zero_sized_fields {
() => {
// Module: crate::check
// Provides: {"bad_non_zero_sized_fields"}
// Dependencies: {}
# [doc = " Emit an error when encountering two or more non-zero-sized fields in a transparent"] # [doc = " enum."] fn bad_non_zero_sized_fields < 'tcx > (tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < 'tcx > , field_count : usize , field_spans : impl Iterator < Item = Span > , sp : Span ,) { if adt . is_enum () { tcx . dcx () . emit_err (errors :: TransparentNonZeroSizedEnum { span : sp , spans : field_spans . collect () , field_count , desc : adt . descr () , }) ; } else { tcx . dcx () . emit_err (errors :: TransparentNonZeroSized { span : sp , spans : field_spans . collect () , field_count , desc : adt . descr () , }) ; } }
};
}
