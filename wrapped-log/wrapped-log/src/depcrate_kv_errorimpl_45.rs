// Generated macro for impl_45 (impl)
macro_rules! Depcrate_kv_errorimpl_45 {
() => {
// Module: crate::kv::error
// Provides: {"impl_45"}
// Dependencies: {}
impl Error { # [doc = " Create an error from a message."] pub fn msg (msg : & 'static str) -> Self { Error { inner : Inner :: Msg (msg) , } } # [cfg (feature = "value-bag")] pub (super) fn from_value (err : crate :: kv :: value :: inner :: Error) -> Self { Error { inner : Inner :: Value (err) , } } # [cfg (feature = "value-bag")] pub (super) fn into_value (self) -> crate :: kv :: value :: inner :: Error { match self . inner { Inner :: Value (err) => err , # [cfg (feature = "kv_std")] _ => crate :: kv :: value :: inner :: Error :: boxed (self) , # [cfg (not (feature = "kv_std"))] _ => crate :: kv :: value :: inner :: Error :: msg ("error inspecting a value") , } } }
};
}
