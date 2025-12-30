// Generated macro for macro_165 (macro)
macro_rules! Depcrate_map_errmacro_165 {
() => {
// Module: crate::map_err
// Provides: {"macro_165"}
// Dependencies: {}
pin_project ! { pub struct MapErrServiceFuture < SF , Req , F , E > where SF : ServiceFactory < Req >, F : Fn (SF :: Error) -> E , { # [pin] fut : SF :: Future , mapper : F , } }
};
}
