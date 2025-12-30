// Generated macro for invalid_varint_discriminant (function)
macro_rules! Depcrate_varint_decode_unsignedinvalid_varint_discriminant {
() => {
// Module: crate::varint::decode_unsigned
// Provides: {"invalid_varint_discriminant"}
// Dependencies: {}
# [inline (never)] # [cold] const fn invalid_varint_discriminant < T > (expected : IntegerType , found : IntegerType ,) -> Result < T , DecodeError > { Err (DecodeError :: InvalidIntegerType { expected , found }) }
};
}
