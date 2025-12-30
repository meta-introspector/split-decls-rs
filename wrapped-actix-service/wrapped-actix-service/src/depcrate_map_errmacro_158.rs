// Generated macro for macro_158 (macro)
macro_rules! Depcrate_map_errmacro_158 {
() => {
// Module: crate::map_err
// Provides: {"macro_158"}
// Dependencies: {}
pin_project ! { pub struct MapErrFuture < A , Req , F , E > where A : Service < Req >, F : Fn (A :: Error) -> E , { f : F , # [pin] fut : A :: Future , } }
};
}
