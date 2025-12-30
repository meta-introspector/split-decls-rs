// Generated macro for impl_240 (impl)
macro_rules! Depcrate_plots_minmaximpl_240 {
() => {
// Module: crate::plots::minmax
// Provides: {"impl_240"}
// Dependencies: {}
impl < Y > XYMinMax < Y > { pub fn init (x_data_range : std :: ops :: Range < f32 > , x_start : Option < f32 > , x_end : Option < f32 > , y_range : std :: ops :: Range < Y > ,) -> Self { let x = XMinMax :: new (x_data_range . start , x_data_range . end , x_start , x_end) ; Self { x , y_range } } }
};
}
