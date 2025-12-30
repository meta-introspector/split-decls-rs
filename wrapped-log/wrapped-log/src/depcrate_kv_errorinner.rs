// Generated macro for Inner (enum)
macro_rules! Depcrate_kv_errorInner {
() => {
// Module: crate::kv::error
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug)] enum Inner { # [cfg (feature = "std")] Boxed (std_support :: BoxedError) , Msg (& 'static str) , # [cfg (feature = "value-bag")] Value (crate :: kv :: value :: inner :: Error) , Fmt , }
};
}
