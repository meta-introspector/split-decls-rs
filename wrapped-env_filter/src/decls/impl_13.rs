macro_rules! deps {
    () => {
        FilteredLog!();
        Filter!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : Log > FilteredLog < T > { # [doc = " Create a new filtered log."] pub fn new (log : T , filter : Filter) -> Self { Self { log , filter } } }
    };
}

impl_13!()