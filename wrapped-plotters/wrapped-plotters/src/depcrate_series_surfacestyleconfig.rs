// Generated macro for StyleConfig (enum)
macro_rules! Depcrate_series_surfaceStyleConfig {
() => {
// Module: crate::series::surface
// Provides: {"StyleConfig"}
// Dependencies: {}
enum StyleConfig < 'a , T > { Fixed (ShapeStyle) , Function (& 'a dyn Fn (& T) -> ShapeStyle) , }
};
}
