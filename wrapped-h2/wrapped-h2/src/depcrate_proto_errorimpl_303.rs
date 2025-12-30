// Generated macro for impl_303 (impl)
macro_rules! Depcrate_proto_errorimpl_303 {
() => {
// Module: crate::proto::error
// Provides: {"impl_303"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: Reset (_ , reason , _) | Self :: GoAway (_ , reason , _) => reason . fmt (fmt) , Self :: Io (_ , Some (ref inner)) => inner . fmt (fmt) , Self :: Io (kind , None) => io :: Error :: from (kind) . fmt (fmt) , } } }
};
}
