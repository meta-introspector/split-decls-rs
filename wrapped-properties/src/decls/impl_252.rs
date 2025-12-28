macro_rules! deps {
    () => {
        PropertyNamesShortBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : NamedEnumeratedProperty > Default for PropertyNamesShortBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_252!()