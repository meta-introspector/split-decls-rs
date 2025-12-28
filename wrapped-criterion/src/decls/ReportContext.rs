macro_rules! deps {
    () => {
        PlotConfiguration!();
    };
}

macro_rules! ReportContext {
    () => {
        deps!();
        pub struct ReportContext { pub output_directory : PathBuf , pub plot_config : PlotConfiguration , }
    };
}

ReportContext!()