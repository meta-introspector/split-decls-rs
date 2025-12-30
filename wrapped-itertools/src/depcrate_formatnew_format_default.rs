// Generated macro for new_format_default (function)
macro_rules! Depcrate_formatnew_format_default {
() => {
// Module: crate::format
// Provides: {"new_format_default"}
// Dependencies: {}
pub fn new_format_default < I > (iter : I , separator : & str) -> Format < '_ , I > where I : Iterator , { Format { sep : separator , inner : Cell :: new (Some (iter)) , } }
};
}
