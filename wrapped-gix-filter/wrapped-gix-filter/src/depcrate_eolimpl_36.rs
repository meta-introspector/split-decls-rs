// Generated macro for impl_36 (impl)
macro_rules! Depcrate_eolimpl_36 {
() => {
// Module: crate::eol
// Provides: {"impl_36"}
// Dependencies: {}
impl From < AutoCrlf > for AttributesDigest { fn from (value : AutoCrlf) -> Self { match value { AutoCrlf :: Input => AttributesDigest :: TextAutoInput , AutoCrlf :: Enabled => AttributesDigest :: TextAutoCrlf , AutoCrlf :: Disabled => AttributesDigest :: Binary , } } }
};
}
