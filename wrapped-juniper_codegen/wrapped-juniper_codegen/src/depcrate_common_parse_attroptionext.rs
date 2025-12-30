// Generated macro for OptionExt (trait)
macro_rules! Depcrate_common_parse_attrOptionExt {
() => {
// Module: crate::common::parse::attr
// Provides: {"OptionExt"}
// Dependencies: {}
# [doc = " Handy extension of [`Option`] methods, used in this crate."] pub (crate) trait OptionExt { type Inner ; # [doc = " Transforms the `Option<T>` into a `Result<(), E>`, mapping `None` to `Ok(())` and `Some(v)`"] # [doc = " to `Err(err(v))`."] fn none_or_else < E , F > (self , err : F) -> Result < () , E > where F : FnOnce (Self :: Inner) -> E ; }
};
}
