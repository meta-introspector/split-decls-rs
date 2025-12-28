macro_rules! deps {
    () => {
        TestRegistry!();
        RegistryBuilder!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [doc = " Setup a local pseudo-crates.io [`TestRegistry`]"] # [doc = ""] # [doc = " This is implicitly called by [`Package::new`]."] # [doc = ""] # [doc = " When calling `cargo publish`, see instead [`crate::publish`]."] pub fn init () -> TestRegistry { RegistryBuilder :: new () . build () }
    };
}

init!();