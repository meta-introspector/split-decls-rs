macro_rules! deps {
    () => {
        PlottingBackend!();
    };
}

macro_rules! default_plotting_backend {
    () => {
        deps!();
        fn default_plotting_backend () -> & 'static PlottingBackend { static DEFAULT_PLOTTING_BACKEND : OnceLock < PlottingBackend > = OnceLock :: new () ; DEFAULT_PLOTTING_BACKEND . get_or_init (| | match gnuplot_version () { Ok (_) => PlottingBackend :: Gnuplot , # [cfg (feature = "plotters")] Err (e) => { match e { VersionError :: Exec (_) => eprintln ! ("Gnuplot not found, using plotters backend") , e => eprintln ! ("Gnuplot not found or not usable, using plotters backend\n{}" , e) , } ; PlottingBackend :: Plotters } # [cfg (not (feature = "plotters"))] Err (_) => PlottingBackend :: None , }) }
    };
}

default_plotting_backend!()