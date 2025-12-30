// Generated macro for Kind (enum)
macro_rules! Depcrate_unitKind {
() => {
// Module: crate::unit
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " Either a static label or a dynamic one implementing [`DisplayValue`]."] # [derive (Clone)] pub enum Kind { # [doc = " Display only the given statically known label."] Label (& 'static str) , # [doc = " Display a label created dynamically."] Dynamic (Arc < dyn DisplayValue + Send + Sync >) , }
};
}
