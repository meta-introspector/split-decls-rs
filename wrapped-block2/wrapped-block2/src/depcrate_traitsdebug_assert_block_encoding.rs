// Generated macro for debug_assert_block_encoding (function)
macro_rules! Depcrate_traitsdebug_assert_block_encoding {
() => {
// Module: crate::traits
// Provides: {"debug_assert_block_encoding"}
// Dependencies: {}
# [doc = " Checks for encoding compatibility between the given generic parameters,"] # [doc = " panicking if it is not, but only on `cfg(debug_assertions)` and if `E` is"] # [doc = " not none."] # [cfg_attr (not (debug_assertions) , inline (always))] # [allow (unused)] pub (crate) fn debug_assert_block_encoding < A , R , E > () where A : EncodeArguments , R : EncodeReturn , E : ManualBlockEncodingExt < Arguments = A , Return = R > , { # [cfg (debug_assertions)] { if ! E :: IS_NONE { assert_eq ! (E :: ENCODING_CSTR , &* crate :: encoding :: block_signature_string ::< A , R > ()) ; } } }
};
}
