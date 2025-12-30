// Generated macro for Filter (struct)
macro_rules! Depcrate_filterFilter {
() => {
// Module: crate::filter
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " A log filter."] # [doc = ""] # [doc = " This struct can be used to determine whether or not a log record"] # [doc = " should be written to the output."] # [doc = " Use the [`Builder`] type to parse and construct a `Filter`."] # [doc = ""] # [doc = " [`Builder`]: struct.Builder.html"] # [derive (Clone)] pub struct Filter { directives : Vec < Directive > , filter : Option < FilterOp > , }
};
}
