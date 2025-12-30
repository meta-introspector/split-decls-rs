// Generated macro for set_SOURCE_DATE (function)
macro_rules! Depcrateset_SOURCE_DATE {
() => {
// Module: crate
// Provides: {"set_SOURCE_DATE"}
// Dependencies: {}
# [doc = " Sets the `SOURCE_DATE` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"2021-04-14Z\"`"] # [doc = ""] # [doc = " Reads the"] # [doc = " [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)"] # [doc = " env var if set.  Otherwise, runs `git` to get the value."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when:"] # [doc = " - `SOURCE_DATE_EPOCH` env var is non-empty and invalid"] # [doc = " - it fails to get the timestamp from `git`"] pub fn set_SOURCE_DATE () -> Result < () , String > { let source_time = get_source_time () ? ; let value = format_date (source_time) ? ; println ! ("cargo:rustc-env=SOURCE_DATE={value}") ; Ok (()) }
};
}
