macro_rules! deps {
    () => {
        OnceLock!();
        Collector!();
    };
}

macro_rules! collector {
    () => {
        deps!();
        fn collector () -> & 'static Collector { # [cfg (not (crossbeam_loom))] { # [doc = " The global data for the default garbage collector."] static COLLECTOR : OnceLock < Collector > = OnceLock :: new () ; COLLECTOR . get_or_init (Collector :: new) } # [cfg (crossbeam_loom)] { loom :: lazy_static ! { # [doc = " The global data for the default garbage collector."] static ref COLLECTOR : Collector = Collector :: new () ; } & COLLECTOR } }
    };
}

collector!()