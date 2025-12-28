macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : NamedEnumeratedProperty > Default for PropertyNamesShortBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_53!()