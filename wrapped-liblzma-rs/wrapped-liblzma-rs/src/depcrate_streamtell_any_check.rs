// Generated macro for TELL_ANY_CHECK (const)
macro_rules! Depcrate_streamTELL_ANY_CHECK {
() => {
// Module: crate::stream
// Provides: {"TELL_ANY_CHECK"}
// Dependencies: {}
# [doc = " A flag passed when initializing a decoder, causes [`Stream::process`] to return"] # [doc = " [`Status::GetCheck`] as soon as the integrity check is known."] pub const TELL_ANY_CHECK : u32 = liblzma_sys :: LZMA_TELL_ANY_CHECK ;
};
}
