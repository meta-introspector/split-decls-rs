// Generated macro for impl_173 (impl)
macro_rules! Depcrate_errorimpl_173 {
() => {
// Module: crate::error
// Provides: {"impl_173"}
// Dependencies: {}
# [cfg (feature = "http1")] impl Parse { # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn content_length_invalid () -> Self { Parse :: Header (Header :: ContentLengthInvalid) } # [cfg (feature = "server")] pub (crate) fn transfer_encoding_invalid () -> Self { Parse :: Header (Header :: TransferEncodingInvalid) } # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn transfer_encoding_unexpected () -> Self { Parse :: Header (Header :: TransferEncodingUnexpected) } }
};
}
