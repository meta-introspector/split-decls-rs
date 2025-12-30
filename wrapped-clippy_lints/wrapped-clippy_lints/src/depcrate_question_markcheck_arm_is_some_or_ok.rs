// Generated macro for check_arm_is_some_or_ok (function)
macro_rules! Depcrate_question_markcheck_arm_is_some_or_ok {
() => {
// Module: crate::question_mark
// Provides: {"check_arm_is_some_or_ok"}
// Dependencies: {}
fn check_arm_is_some_or_ok < 'tcx > (cx : & LateContext < 'tcx > , mode : TryMode , arm : & Arm < 'tcx >) -> bool { let happy_ctor = match mode { TryMode :: Result => ResultOk , TryMode :: Option => OptionSome , } ; if arm . guard . is_none () && let Some (val_binding) = extract_ctor_call (cx , happy_ctor , arm . pat) && let Some (binding) = extract_binding_pat (val_binding) && peel_blocks (arm . body) . res_local_id () == Some (binding) { true } else { false } }
};
}
