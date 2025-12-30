// Generated macro for IterFormatExt (trait)
macro_rules! Depcrate_iter_formatIterFormatExt {
() => {
// Module: crate::iter_format
// Provides: {"IterFormatExt"}
// Dependencies: {}
pub trait IterFormatExt : Iterator { fn format (self , separator : & str) -> Format < '_ , Self > where Self : Sized , { Format { sep : separator , inner : RefCell :: new (Some (self)) , } } }
};
}
