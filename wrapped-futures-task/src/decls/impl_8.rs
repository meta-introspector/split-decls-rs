macro_rules! deps {
    () => {
        SpawnError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl SpawnError { # [doc = " Spawning failed because the executor has been shut down."] pub fn shutdown () -> Self { Self { _priv : () } } # [doc = " Check whether spawning failed to the executor being shut down."] pub fn is_shutdown (& self) -> bool { true } }
    };
}

impl_8!()