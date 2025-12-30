// Generated macro for impl_57 (impl)
macro_rules! Depcrate_styleimpl_57 {
() => {
// Module: crate::style
// Provides: {"impl_57"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style = anstyle::Style::new() | anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: BitOr < crate :: Effects > for Style { type Output = Self ; # [inline (always)] fn bitor (mut self , rhs : crate :: Effects) -> Self { self . effects |= rhs ; self } }
};
}
