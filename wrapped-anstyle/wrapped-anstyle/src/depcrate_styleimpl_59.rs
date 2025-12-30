// Generated macro for impl_59 (impl)
macro_rules! Depcrate_styleimpl_59 {
() => {
// Module: crate::style
// Provides: {"impl_59"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style = anstyle::Style::new().bold().underline() - anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: Sub < crate :: Effects > for Style { type Output = Self ; # [inline] fn sub (mut self , other : crate :: Effects) -> Self { self . effects -= other ; self } }
};
}
