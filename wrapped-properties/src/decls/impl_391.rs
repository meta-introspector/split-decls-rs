macro_rules! deps {
    () => {
        ScriptWithExtensionsBorrowed!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for ScriptWithExtensionsBorrowed < 'static > { fn default () -> Self { Self :: new () } }
    };
}

impl_391!();