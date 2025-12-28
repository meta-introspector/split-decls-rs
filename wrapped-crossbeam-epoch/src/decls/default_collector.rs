macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! default_collector {
    () => {
        deps!();
        # [doc = " Returns the default global collector."] pub fn default_collector () -> & 'static Collector { collector () }
    };
}

default_collector!();