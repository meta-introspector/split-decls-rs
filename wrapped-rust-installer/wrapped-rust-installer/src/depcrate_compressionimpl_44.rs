// Generated macro for impl_44 (impl)
macro_rules! Depcrate_compressionimpl_44 {
() => {
// Module: crate::compression
// Provides: {"impl_44"}
// Dependencies: {}
impl FromStr for CompressionProfile { type Err = Error ; fn from_str (input : & str) -> Result < Self , Error > { Ok (match input { "fast" => Self :: Fast , "balanced" => Self :: Balanced , "best" => Self :: Best , "no-op" => Self :: NoOp , other => anyhow :: bail ! ("invalid compression profile: {other}") , }) } }
};
}
