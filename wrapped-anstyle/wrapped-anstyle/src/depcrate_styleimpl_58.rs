// Generated macro for impl_58 (impl)
macro_rules! Depcrate_styleimpl_58 {
() => {
// Module: crate::style
// Provides: {"impl_58"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut style = anstyle::Style::new();"] # [doc = " style |= anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: BitOrAssign < crate :: Effects > for Style { # [inline] fn bitor_assign (& mut self , other : crate :: Effects) { self . effects |= other ; } }
};
}
