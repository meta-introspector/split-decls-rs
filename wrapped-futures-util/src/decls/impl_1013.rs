macro_rules! deps {
    () => {
        Sink01CompatExt!();
    };
}

macro_rules! impl_1013 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < Si : Sink01 > Sink01CompatExt for Si { }
    };
}

impl_1013!();