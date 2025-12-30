// Generated macro for impl_25 (impl)
macro_rules! Depcrate_contextimpl_25 {
() => {
// Module: crate::context
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , E > WrapErr < T , E > for Result < T , E > where E : ext :: StdError + Send + Sync + 'static , { fn wrap_err < D > (self , msg : D) -> Result < T , Report > where D : Display + Send + Sync + 'static , { match self { Ok (t) => Ok (t) , Err (e) => Err (e . ext_report (msg)) , } } fn wrap_err_with < D , F > (self , msg : F) -> Result < T , Report > where D : Display + Send + Sync + 'static , F : FnOnce () -> D , { match self { Ok (t) => Ok (t) , Err (e) => Err (e . ext_report (msg ())) , } } }
};
}
