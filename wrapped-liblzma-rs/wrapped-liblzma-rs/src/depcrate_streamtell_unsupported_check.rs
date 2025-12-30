// Generated macro for TELL_UNSUPPORTED_CHECK (const)
macro_rules! Depcrate_streamTELL_UNSUPPORTED_CHECK {
() => {
// Module: crate::stream
// Provides: {"TELL_UNSUPPORTED_CHECK"}
// Dependencies: {}
# [doc = " A flag passed when initializing a decoder, causes [`Stream::process`] to return"] # [doc = " [`Error::UnsupportedCheck`] if the stream being decoded has an integrity check"] # [doc = " that cannot be verified by this build of liblzma."] pub const TELL_UNSUPPORTED_CHECK : u32 = liblzma_sys :: LZMA_TELL_UNSUPPORTED_CHECK ;
};
}
