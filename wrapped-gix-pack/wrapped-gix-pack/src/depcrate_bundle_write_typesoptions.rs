// Generated macro for Options (struct)
macro_rules! Depcrate_bundle_write_typesOptions {
() => {
// Module: crate::bundle::write::types
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Configuration for [`write_to_directory`][crate::Bundle::write_to_directory()] or"] # [doc = " [`write_to_directory_eagerly`][crate::Bundle::write_to_directory_eagerly()]"] # [derive (Debug , Clone)] pub struct Options { # [doc = " The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used."] pub thread_limit : Option < usize > , # [doc = " Determine how much processing to spend on protecting against corruption or recovering from errors."] pub iteration_mode : crate :: data :: input :: Mode , # [doc = " The version of pack index to write, should be [`crate::index::Version::default()`]"] pub index_version : crate :: index :: Version , # [doc = " The kind of hash to use when writing the bundle."] pub object_hash : gix_hash :: Kind , }
};
}
