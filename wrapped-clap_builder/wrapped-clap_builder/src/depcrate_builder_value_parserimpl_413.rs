// Generated macro for impl_413 (impl)
macro_rules! Depcrate_builder_value_parserimpl_413 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_413"}
// Dependencies: {}
impl < P , F , T > MapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> T + Clone , T : Send + Sync + Clone , { fn new (parser : P , func : F) -> Self { Self { parser , func } } }
};
}
