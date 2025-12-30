// Generated macro for impl_788 (impl)
macro_rules! Depcrate_series_surfaceimpl_788 {
() => {
// Module: crate::series::surface
// Provides: {"impl_788"}
// Dependencies: {}
impl < T > StyleConfig < '_ , T > { fn get_style (& self , v : & T) -> ShapeStyle { match self { StyleConfig :: Fixed (s) => * s , StyleConfig :: Function (f) => f (v) , } } }
};
}
