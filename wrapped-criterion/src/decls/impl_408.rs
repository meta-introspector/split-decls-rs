macro_rules! deps {
    () => {
        AxisScale!();
        PlotConfiguration!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl PlotConfiguration { # [must_use] # [doc = " Set the axis scale ([linear] or [logarithmic]) for the summary plots."] # [doc = ""] # [doc = " Typically, you would set this to logarithmic if benchmarking over a"] # [doc = " range of inputs which scale exponentially. Defaults to [`AxisScale::Linear`]."] # [doc = ""] # [doc = " [linear]: AxisScale::Linear"] # [doc = " [logarithmic]: AxisScale::Logarithmic"] pub fn summary_scale (mut self , new_scale : AxisScale) -> PlotConfiguration { self . summary_scale = new_scale ; self } }
    };
}

impl_408!()