macro_rules! deps {
    () => {
        PropertyParserBorrowed!();
        ParseableEnumeratedProperty!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl < T : ParseableEnumeratedProperty > Default for PropertyParserBorrowed < 'static , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_231!();