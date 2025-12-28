macro_rules! deps {
    () => {
        EnumeratedProperty!();
        CodePointMapDataBorrowed!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : EnumeratedProperty > Default for CodePointMapDataBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_14!();