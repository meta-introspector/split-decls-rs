// Generated macro for Options (struct)
macro_rules! Depcrate_usage_optionsOptions {
() => {
// Module: crate::usage::options
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Control struct for searching type parameters."] # [doc = ""] # [doc = " This acts as the search context, preserving information that might have been"] # [doc = " kept on a visitor in a different implementation."] # [doc = " Trait implementers are required to pass this through on any invocations they make."] # [doc = ""] # [doc = " # Usage"] # [doc = " For extensibility, `Options` hides all of its fields from consumers."] # [doc = " To create an instance, use the `From<Purpose>` trait implementation:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use darling_core::usage::{Options, Purpose};"] # [doc = " let opts: Options = Purpose::BoundImpl.into();"] # [doc = " assert!(!opts.include_type_path_qself());"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Options { purpose : Purpose , # [doc (hidden)] __nonexhaustive : () , }
};
}
