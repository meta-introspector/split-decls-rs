// Generated macro for macro_249 (macro)
macro_rules! Depcrate_transform_errmacro_249 {
() => {
// Module: crate::transform_err
// Provides: {"macro_249"}
// Dependencies: {}
pin_project ! { pub struct TransformMapInitErrFuture < T , S , F , E , Req > where T : Transform < S , Req >, F : Fn (T :: InitError) -> E , { # [pin] fut : T :: Future , f : F , } }
};
}
