macro_rules! deps {
    () => {
        TestRegistry!();
        RegistryBuilder!();
    };
}

macro_rules! alt_init {
    () => {
        deps!();
        # [doc = " Setup a local \"alternative\" [`TestRegistry`]"] # [doc = ""] # [doc = " When calling `cargo publish`, see instead [`crate::publish`]."] pub fn alt_init () -> TestRegistry { init () ; RegistryBuilder :: new () . alternative () . build () }
    };
}

alt_init!()