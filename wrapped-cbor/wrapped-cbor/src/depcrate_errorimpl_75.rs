// Generated macro for impl_75 (impl)
macro_rules! Depcrate_errorimpl_75 {
() => {
// Module: crate::error
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl From < core :: fmt :: Error > for Error { fn from (_ : core :: fmt :: Error) -> Error { Error (ErrorImpl { code : ErrorCode :: Message , offset : 0 , }) } }
};
}
