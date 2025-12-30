// Generated macro for impl_259 (impl)
macro_rules! Depcrate_plots_packet_sentimpl_259 {
() => {
// Module: crate::plots::packet_sent
// Provides: {"impl_259"}
// Dependencies: {}
impl XYMinMax { fn init (params : & PlotParameters , ss : & SeriesStore , y_max : u64) -> Self { let x = XMinMax :: new (ss . sent_x_min , ss . sent_x_max , params . clamp . start , params . clamp . end ,) ; Self { x , y_min : 0 , y_max } } fn y_range (& self) -> std :: ops :: Range < u64 > { self . y_min .. self . y_max } }
};
}
