// Generated macro for impl_26 (impl)
macro_rules! Depcrate_contextimpl_26 {
() => {
// Module: crate::context
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "anyhow")] impl < T , E > crate :: ContextCompat < T > for Result < T , E > where Self : WrapErr < T , E > , { # [track_caller] fn context < D > (self , msg : D) -> crate :: Result < T , Report > where D : Display + Send + Sync + 'static , { self . wrap_err (msg) } # [track_caller] fn with_context < D , F > (self , f : F) -> crate :: Result < T , Report > where D : Display + Send + Sync + 'static , F : FnOnce () -> D , { self . wrap_err_with (f) } }
};
}
