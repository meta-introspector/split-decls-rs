// Generated macro for impl_578 (impl)
macro_rules! Depcrate_errorimpl_578 {
() => {
// Module: crate::error
// Provides: {"impl_578"}
// Dependencies: {}
impl DecodeError { # [doc = " If the current error is `InvalidIntegerType`, change the `expected` and"] # [doc = " `found` values from `Ux` to `Ix`. This is needed to have correct error"] # [doc = " reporting in src/varint/decode_signed.rs since this calls"] # [doc = " src/varint/decode_unsigned.rs and needs to correct the `expected` and"] # [doc = " `found` types."] pub (crate) fn change_integer_type_to_signed (self) -> DecodeError { match self { Self :: InvalidIntegerType { expected , found } => Self :: InvalidIntegerType { expected : expected . into_signed () , found : found . into_signed () , } , other => other , } } }
};
}
