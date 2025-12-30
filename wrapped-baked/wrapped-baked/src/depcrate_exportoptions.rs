// Generated macro for Options (struct)
macro_rules! Depcrate_exportOptions {
() => {
// Module: crate::export
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for configuring the output of [`BakedExporter`]."] # [non_exhaustive] # [derive (Debug , Clone , Copy)] pub struct Options { # [doc = " By default, baked providers perform fallback internally. This field can be used to"] # [doc = " disable this behavior."] pub use_internal_fallback : bool , # [doc = " Whether to run `rustfmt` on the generated files."] pub pretty : bool , # [doc = " Whether to use separate crates to name types instead of the `icu` metacrate."] # [doc = ""] # [doc = " By default, types will be named through the `icu` crate, like `icu::list::provider::ListJoinerPattern`."] # [doc = " With this enabled, the alternative name from the component crates will be used: `icu_list::provider::ListJoinerPattern`."] # [doc = " This is required when you are not using the `icu` crate, *and* you're building custom data providers;"] # [doc = " data for `compiled_data` constructors uses `icu` names."] pub use_separate_crates : bool , # [doc = " Whether to overwrite existing data. By default, errors if it is present."] pub overwrite : bool , }
};
}
