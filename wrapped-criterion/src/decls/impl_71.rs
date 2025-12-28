macro_rules! deps {
    () => {
        PlotConfiguration!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < & crate :: PlotConfiguration > for PlotConfiguration { fn from (other : & crate :: PlotConfiguration) -> Self { PlotConfiguration { summary_scale : other . summary_scale . into () , } } }
    };
}

impl_71!();