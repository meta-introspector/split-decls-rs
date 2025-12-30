// Generated macro for MetadataBuilder (struct)
macro_rules! DepcrateMetadataBuilder {
() => {
// Module: crate
// Provides: {"MetadataBuilder"}
// Dependencies: {}
# [doc = " Builder for [`Metadata`](struct.Metadata.html)."] # [doc = ""] # [doc = " Typically should only be used by log library creators or for testing and \"shim loggers\"."] # [doc = " The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns"] # [doc = " the created object when `build` is called."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let target = \"myApp\";"] # [doc = " use log::{Level, MetadataBuilder};"] # [doc = " let metadata = MetadataBuilder::new()"] # [doc = "                     .level(Level::Debug)"] # [doc = "                     .target(target)"] # [doc = "                     .build();"] # [doc = " ```"] # [derive (Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct MetadataBuilder < 'a > { metadata : Metadata < 'a > , }
};
}
