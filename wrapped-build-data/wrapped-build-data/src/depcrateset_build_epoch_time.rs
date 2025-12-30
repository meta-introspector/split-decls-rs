// Generated macro for set_BUILD_EPOCH_TIME (function)
macro_rules! Depcrateset_BUILD_EPOCH_TIME {
() => {
// Module: crate
// Provides: {"set_BUILD_EPOCH_TIME"}
// Dependencies: {}
# [doc = " Sets the `BUILD_EPOCH_TIME` env variable, with the current time."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Calling this will make your build"] # [doc = " [non-reproducible](https://reproducible-builds.org/docs/timestamps/)."] # [doc = ""] # [doc = " Example value: `\"1618370707\"`"] pub fn set_BUILD_EPOCH_TIME () { let value = now () ; println ! ("cargo:rustc-env=BUILD_EPOCH_TIME={value}") ; }
};
}
