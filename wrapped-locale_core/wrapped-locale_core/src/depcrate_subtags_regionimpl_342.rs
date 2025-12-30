// Generated macro for impl_342 (impl)
macro_rules! Depcrate_subtags_regionimpl_342 {
() => {
// Module: crate::subtags::region
// Provides: {"impl_342"}
// Dependencies: {}
impl Region { # [doc = " Returns true if the Region has an alphabetic code."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::region;"] # [doc = ""] # [doc = " assert!(region!(\"us\").is_alphabetic());"] # [doc = " ```"] pub fn is_alphabetic (& self) -> bool { self . 0 . len () == 2 } }
};
}
