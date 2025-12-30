// Generated macro for impl_61 (impl)
macro_rules! Depcrate_styleimpl_61 {
() => {
// Module: crate::style
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = anstyle::Effects::BOLD;"] # [doc = " assert_eq!(anstyle::Style::new().effects(effects), effects);"] # [doc = " assert_ne!(anstyle::Effects::UNDERLINE | effects, effects);"] # [doc = " assert_ne!(anstyle::RgbColor(0, 0, 0).on_default() | effects, effects);"] # [doc = " ```"] impl PartialEq < crate :: Effects > for Style { # [inline] fn eq (& self , other : & crate :: Effects) -> bool { let other = Self :: from (* other) ; * self == other } }
};
}
