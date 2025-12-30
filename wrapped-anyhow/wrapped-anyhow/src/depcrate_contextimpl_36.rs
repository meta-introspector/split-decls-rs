// Generated macro for impl_36 (impl)
macro_rules! Depcrate_contextimpl_36 {
() => {
// Module: crate::context
// Provides: {"impl_36"}
// Dependencies: {}
impl < T , E > Context < T , E > for Result < T , E > where E : ext :: StdError + Send + Sync + 'static , { fn context < C > (self , context : C) -> Result < T , Error > where C : Display + Send + Sync + 'static , { match self { Ok (ok) => Ok (ok) , Err (error) => Err (error . ext_context (context)) , } } fn with_context < C , F > (self , context : F) -> Result < T , Error > where C : Display + Send + Sync + 'static , F : FnOnce () -> C , { match self { Ok (ok) => Ok (ok) , Err (error) => Err (error . ext_context (context ())) , } } }
};
}
