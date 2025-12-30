// Generated macro for impl_27 (impl)
macro_rules! Depcrate_contextimpl_27 {
() => {
// Module: crate::context
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "anyhow")] impl < T > crate :: ContextCompat < T > for Option < T > { # [track_caller] fn context < D > (self , msg : D) -> Result < T , Report > where D : Display + Send + Sync + 'static , { match self { Some (t) => Ok (t) , None => Err (Report :: from_display (msg)) , } } # [track_caller] fn with_context < D , F > (self , msg : F) -> Result < T , Report > where D : Display + Send + Sync + 'static , F : FnOnce () -> D , { match self { Some (t) => Ok (t) , None => Err (Report :: from_display (msg ())) , } } }
};
}
