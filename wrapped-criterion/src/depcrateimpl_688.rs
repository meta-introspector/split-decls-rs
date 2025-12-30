// Generated macro for impl_688 (impl)
macro_rules! Depcrateimpl_688 {
() => {
// Module: crate
// Provides: {"impl_688"}
// Dependencies: {}
impl PlottingBackend { fn create_plotter (& self) -> Option < Box < dyn Plotter > > { match self { PlottingBackend :: Gnuplot => Some (Box :: < Gnuplot > :: default ()) , # [cfg (feature = "plotters")] PlottingBackend :: Plotters => Some (Box :: < PlottersBackend > :: default ()) , # [cfg (not (feature = "plotters"))] PlottingBackend :: Plotters => panic ! ("Criterion was built without plotters support.") , PlottingBackend :: None => None , } } }
};
}
