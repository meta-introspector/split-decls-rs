// Generated macro for Override (enum)
macro_rules! Depcrate_util_over_rideOverride {
() => {
// Module: crate::util::over_ride
// Provides: {"Override"}
// Dependencies: {}
# [doc = " A value which can inherit a default value or have an explicit value specified."] # [doc = ""] # [doc = " # Usage"] # [doc = " This type is meant for attributes like `default` in `darling`, which can take the following forms:"] # [doc = ""] # [doc = " * `#[darling(default)]`"] # [doc = " * `#[darling(default=\"path::to::fn\")]`"] # [doc = ""] # [doc = " In a struct collecting input for this attribute, that would be written as:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use darling::{util::Override, FromField};"] # [doc = " #[derive(FromField)]"] # [doc = " #[darling(attributes(darling))]"] # [doc = " pub struct Options {"] # [doc = "    default: Option<Override<syn::Path>>,"] # [doc = " }"] # [doc = ""] # [doc = " impl Options {"] # [doc = "     fn hydrate(self) -> Option<syn::Path> {"] # [doc = "         self.default.map(|ov| ov.unwrap_or(syn::parse_path(\"::Default::default\").unwrap()))"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The `word` format (with no associated value), would produce `Override::Inherit`, while a list"] # [doc = " or value format would produce `Override::Explicit`."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum Override < T > { # [doc = " Inherit the eventual value from an external source."] Inherit , # [doc = " Explicitly set the value."] Explicit (T) , }
};
}
