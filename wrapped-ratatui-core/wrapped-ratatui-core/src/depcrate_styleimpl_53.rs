// Generated macro for impl_53 (impl)
macro_rules! Depcrate_styleimpl_53 {
() => {
// Module: crate::style
// Provides: {"impl_53"}
// Dependencies: {}
# [doc = " A custom debug implementation that prints only the fields that are not the default, and unwraps"] # [doc = " the `Option`s."] impl fmt :: Debug for Style { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("Style::new()") ? ; self . fmt_stylize (f) ? ; Ok (()) } }
};
}
