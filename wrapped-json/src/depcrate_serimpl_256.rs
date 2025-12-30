// Generated macro for impl_256 (impl)
macro_rules! Depcrate_serimpl_256 {
() => {
// Module: crate::ser
// Provides: {"impl_256"}
// Dependencies: {}
impl < W > Serializer < W > where W : io :: Write , { # [doc = " Creates a new JSON serializer."] # [inline] pub fn new (writer : W) -> Self { Serializer :: with_formatter (writer , CompactFormatter) } }
};
}
