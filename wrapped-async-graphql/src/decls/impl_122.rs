macro_rules! deps {
    () => {
        SchemaEnv!();
        SchemaEnvInner!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Deref for SchemaEnv { type Target = SchemaEnvInner ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_122!();