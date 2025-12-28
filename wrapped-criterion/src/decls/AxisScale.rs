macro_rules! AxisScale {
    () => {
        # [doc = " Axis scaling type. Specified via [`PlotConfiguration::summary_scale`]."] # [derive (Debug , Default , Clone , Copy)] pub enum AxisScale { # [doc = " Axes scale linearly"] # [default] Linear , # [doc = " Axes scale logarithmically"] Logarithmic , }
    };
}

AxisScale!()