// Generated macro for impl_416 (impl)
macro_rules! Depcrate_builder_value_parserimpl_416 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_416"}
// Dependencies: {}
impl < P , F , T , E > TryMapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> Result < T , E > + Clone + Send + Sync + 'static , T : Send + Sync + Clone , E : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { fn new (parser : P , func : F) -> Self { Self { parser , func } } }
};
}
