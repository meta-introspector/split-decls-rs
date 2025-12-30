// Generated macro for impl_118 (impl)
macro_rules! Depcrate_flycheckimpl_118 {
() => {
// Module: crate::flycheck
// Provides: {"impl_118"}
// Dependencies: {}
impl fmt :: Debug for FlycheckMessage { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FlycheckMessage :: AddDiagnostic { id , generation , workspace_root , diagnostic , package_id , } => f . debug_struct ("AddDiagnostic") . field ("id" , id) . field ("generation" , generation) . field ("workspace_root" , workspace_root) . field ("package_id" , package_id) . field ("diagnostic_code" , & diagnostic . code . as_ref () . map (| it | & it . code)) . finish () , FlycheckMessage :: ClearDiagnostics { id , kind } => { f . debug_struct ("ClearDiagnostics") . field ("id" , id) . field ("kind" , kind) . finish () } FlycheckMessage :: Progress { id , progress } => { f . debug_struct ("Progress") . field ("id" , id) . field ("progress" , progress) . finish () } } } }
};
}
