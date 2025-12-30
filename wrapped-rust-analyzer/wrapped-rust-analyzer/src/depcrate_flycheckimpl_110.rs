// Generated macro for impl_110 (impl)
macro_rules! Depcrate_flycheckimpl_110 {
() => {
// Module: crate::flycheck
// Provides: {"impl_110"}
// Dependencies: {}
impl fmt :: Debug for FlycheckMessage { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FlycheckMessage :: AddDiagnostic { id , workspace_root , diagnostic , package_id } => f . debug_struct ("AddDiagnostic") . field ("id" , id) . field ("workspace_root" , workspace_root) . field ("package_id" , package_id) . field ("diagnostic_code" , & diagnostic . code . as_ref () . map (| it | & it . code)) . finish () , FlycheckMessage :: ClearDiagnostics { id , package_id } => f . debug_struct ("ClearDiagnostics") . field ("id" , id) . field ("package_id" , package_id) . finish () , FlycheckMessage :: Progress { id , progress } => { f . debug_struct ("Progress") . field ("id" , id) . field ("progress" , progress) . finish () } } } }
};
}
