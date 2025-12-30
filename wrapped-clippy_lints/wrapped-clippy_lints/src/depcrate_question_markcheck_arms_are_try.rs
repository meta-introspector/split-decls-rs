// Generated macro for check_arms_are_try (function)
macro_rules! Depcrate_question_markcheck_arms_are_try {
() => {
// Module: crate::question_mark
// Provides: {"check_arms_are_try"}
// Dependencies: {}
fn check_arms_are_try < 'tcx > (cx : & LateContext < 'tcx > , mode : TryMode , arm1 : & Arm < 'tcx > , arm2 : & Arm < 'tcx >) -> bool { (check_arm_is_some_or_ok (cx , mode , arm1) && check_arm_is_none_or_err (cx , mode , arm2)) || (check_arm_is_some_or_ok (cx , mode , arm2) && check_arm_is_none_or_err (cx , mode , arm1)) }
};
}
