// Generated macro for impl_86 (impl)
macro_rules! Depcrate_fmt_kvimpl_86 {
() => {
// Module: crate::fmt::kv
// Provides: {"impl_86"}
// Dependencies: {}
impl DefaultVisitSource < '_ > { fn style_key < 'k > (& self , text : Key < 'k >) -> StyledValue < Key < 'k > > { # [cfg (feature = "color")] { StyledValue { style : if self . 0 . write_style == WriteStyle :: Never { Style :: new () } else { Style :: new () . italic () } , value : text , } } # [cfg (not (feature = "color"))] { text } } }
};
}
