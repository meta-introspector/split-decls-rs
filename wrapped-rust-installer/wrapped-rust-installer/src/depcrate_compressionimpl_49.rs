// Generated macro for impl_49 (impl)
macro_rules! Depcrate_compressionimpl_49 {
() => {
// Module: crate::compression
// Provides: {"impl_49"}
// Dependencies: {}
impl TryFrom < & '_ str > for CompressionFormats { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let mut parsed = Vec :: new () ; for format in value . split (',') { match format . trim () { "gz" => parsed . push (CompressionFormat :: Gz) , "xz" => parsed . push (CompressionFormat :: Xz) , other => anyhow :: bail ! ("unknown compression format: {}" , other) , } } Ok (CompressionFormats (parsed)) } }
};
}
