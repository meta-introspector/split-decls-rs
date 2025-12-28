macro_rules! deps {
    () => {
        QueryEnv!();
        QueryEnvInner!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl Deref for QueryEnv { type Target = QueryEnvInner ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_348!();