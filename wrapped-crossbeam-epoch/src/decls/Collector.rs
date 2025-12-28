macro_rules! deps {
    () => {
        Global!();
    };
}

macro_rules! Collector {
    () => {
        deps!();
        # [doc = " An epoch-based garbage collector."] pub struct Collector { pub (crate) global : Arc < Global > , }
    };
}

Collector!();