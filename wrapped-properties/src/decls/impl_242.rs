macro_rules! deps {
    () => {
        PropertyNamesLongBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : NamedEnumeratedProperty > Default for PropertyNamesLongBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_242!();