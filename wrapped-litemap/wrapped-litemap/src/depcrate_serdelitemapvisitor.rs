// Generated macro for LiteMapVisitor (struct)
macro_rules! Depcrate_serdeLiteMapVisitor {
() => {
// Module: crate::serde
// Provides: {"LiteMapVisitor"}
// Dependencies: {}
# [doc = " Modified example from https://serde.rs/deserialize-map.html"] # [expect (clippy :: type_complexity)] struct LiteMapVisitor < K , V , R > { marker : PhantomData < fn () -> LiteMap < K , V , R > > , }
};
}
