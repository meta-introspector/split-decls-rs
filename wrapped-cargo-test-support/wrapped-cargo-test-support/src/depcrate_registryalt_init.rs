// Generated macro for alt_init (function)
macro_rules! Depcrate_registryalt_init {
() => {
// Module: crate::registry
// Provides: {"alt_init"}
// Dependencies: {}
# [doc = " Setup a local \"alternative\" [`TestRegistry`]"] # [doc = ""] # [doc = " When calling `cargo publish`, see instead [`crate::publish`]."] pub fn alt_init () -> TestRegistry { init () ; RegistryBuilder :: new () . alternative () . build () }
};
}
