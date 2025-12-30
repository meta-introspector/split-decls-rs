// Generated macro for impl_220 (impl)
macro_rules! Depcrate_plots_conn_overviewimpl_220 {
() => {
// Module: crate::plots::conn_overview
// Provides: {"impl_220"}
// Dependencies: {}
impl XYMinMax { fn init (params : & PlotParameters , ss : & SeriesStore , y_max : u64) -> Self { let x = XMinMax :: new (ss . sent_x_min , ss . sent_x_max , params . clamp . start , params . clamp . end ,) ; Self { x , y_min : 0 , y_max } } fn y_range (& self) -> std :: ops :: Range < u64 > { self . y_min .. self . y_max } }
};
}
