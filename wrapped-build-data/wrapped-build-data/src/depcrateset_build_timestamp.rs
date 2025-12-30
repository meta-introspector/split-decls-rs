// Generated macro for set_BUILD_TIMESTAMP (function)
macro_rules! Depcrateset_BUILD_TIMESTAMP {
() => {
// Module: crate
// Provides: {"set_BUILD_TIMESTAMP"}
// Dependencies: {}
# [doc = " Sets the `BUILD_TIMESTAMP` env variable, with the current date & time, in UTC."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Calling this will make your build"] # [doc = " [non-reproducible](https://reproducible-builds.org/docs/timestamps/)."] # [doc = ""] # [doc = " Example value: `\"2021-04-14T03:25:07Z\"`"] # [allow (clippy :: missing_panics_doc)] pub fn set_BUILD_TIMESTAMP () { let value = format_timestamp (now ()) . unwrap () ; println ! ("cargo:rustc-env=BUILD_TIMESTAMP={value}") ; }
};
}
