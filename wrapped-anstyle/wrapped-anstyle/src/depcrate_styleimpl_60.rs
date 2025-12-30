// Generated macro for impl_60 (impl)
macro_rules! Depcrate_styleimpl_60 {
() => {
// Module: crate::style
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut style = anstyle::Style::new().bold().underline();"] # [doc = " style -= anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: SubAssign < crate :: Effects > for Style { # [inline] fn sub_assign (& mut self , other : crate :: Effects) { self . effects -= other ; } }
};
}
