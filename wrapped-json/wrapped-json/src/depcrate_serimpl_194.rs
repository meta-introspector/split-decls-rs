// Generated macro for impl_194 (impl)
macro_rules! Depcrate_serimpl_194 {
() => {
// Module: crate::ser
// Provides: {"impl_194"}
// Dependencies: {}
impl < W > Serializer < W > where W : io :: Write , { # [doc = " Creates a new JSON serializer."] # [inline] pub fn new (writer : W) -> Self { Serializer :: with_formatter (writer , CompactFormatter) } }
};
}
