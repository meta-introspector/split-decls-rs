// Generated macro for macro_133 (macro)
macro_rules! Depcrate_mapmacro_133 {
() => {
// Module: crate::map
// Provides: {"macro_133"}
// Dependencies: {}
pin_project ! { pub struct MapServiceFuture < A , F , Req , Res > where A : ServiceFactory < Req >, F : FnMut (A :: Response) -> Res , { # [pin] fut : A :: Future , f : Option < F >, } }
};
}
