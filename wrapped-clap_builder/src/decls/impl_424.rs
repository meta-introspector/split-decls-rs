macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [cfg (not (feature = "debug"))] impl Backtrace { fn new () -> Option < Self > { None } }
    };
}

impl_424!();