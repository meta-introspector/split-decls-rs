macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        # [cfg (feature = "debug")] impl Backtrace { fn new () -> Option < Self > { Some (Self (backtrace :: Backtrace :: new ())) } }
    };
}

impl_421!()