macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! gnuplot_version {
    () => {
        deps!();
        fn gnuplot_version () -> & 'static Result < Version , VersionError > { static GNUPLOT_VERSION : OnceLock < Result < Version , VersionError > > = OnceLock :: new () ; GNUPLOT_VERSION . get_or_init (criterion_plot :: version) }
    };
}

gnuplot_version!()