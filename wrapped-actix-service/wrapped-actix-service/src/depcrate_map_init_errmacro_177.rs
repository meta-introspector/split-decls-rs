// Generated macro for macro_177 (macro)
macro_rules! Depcrate_map_init_errmacro_177 {
() => {
// Module: crate::map_init_err
// Provides: {"macro_177"}
// Dependencies: {}
pin_project ! { pub struct MapInitErrFuture < A , F , Req , E > where A : ServiceFactory < Req >, F : Fn (A :: InitError) -> E , { f : F , # [pin] fut : A :: Future , } }
};
}
