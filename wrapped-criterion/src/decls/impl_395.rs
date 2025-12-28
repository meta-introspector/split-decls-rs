macro_rules! deps {
    () => {
        Plotter!();
        Criterion!();
        Gnuplot!();
        PlottingBackend!();
        PlottersBackend!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl PlottingBackend { fn create_plotter (& self) -> Option < Box < dyn Plotter > > { match self { PlottingBackend :: Gnuplot => Some (Box :: < Gnuplot > :: default ()) , # [cfg (feature = "plotters")] PlottingBackend :: Plotters => Some (Box :: < PlottersBackend > :: default ()) , # [cfg (not (feature = "plotters"))] PlottingBackend :: Plotters => panic ! ("Criterion was built without plotters support.") , PlottingBackend :: None => None , } } }
    };
}

impl_395!();