macro_rules! deps {
    () => {
        ParseableEnumeratedProperty!();
        PropertyParserBorrowed!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : ParseableEnumeratedProperty > Default for PropertyParserBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_32!();