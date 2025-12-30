// Generated macro for impl_56 (impl)
macro_rules! Depcrate_styleimpl_56 {
() => {
// Module: crate::style
// Provides: {"impl_56"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style: anstyle::Style = anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl From < crate :: Effects > for Style { # [inline] fn from (effects : crate :: Effects) -> Self { Self :: new () . effects (effects) } }
};
}
