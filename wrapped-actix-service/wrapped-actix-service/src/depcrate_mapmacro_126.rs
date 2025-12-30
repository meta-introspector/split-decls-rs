// Generated macro for macro_126 (macro)
macro_rules! Depcrate_mapmacro_126 {
() => {
// Module: crate::map
// Provides: {"macro_126"}
// Dependencies: {}
pin_project ! { pub struct MapFuture < A , F , Req , Res > where A : Service < Req >, F : FnMut (A :: Response) -> Res , { f : F , # [pin] fut : A :: Future , } }
};
}
