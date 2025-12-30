// Generated macro for impl_283 (impl)
macro_rules! Depcrate_sparklineimpl_283 {
() => {
// Module: crate::sparkline
// Provides: {"impl_283"}
// Dependencies: {}
impl SparklineBar { # [doc = " Sets the style of the bar."] # [doc = ""] # [doc = " `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or"] # [doc = " your own type that implements [`Into<Style>`])."] # [doc = ""] # [doc = " If not set, the default style of the sparkline will be used."] # [doc = ""] # [doc = " As well as the style of the sparkline, each [`SparklineBar`] may optionally set its own"] # [doc = " style.  If set, the style of the bar will be the style of the sparkline combined with"] # [doc = " the style of the bar."] # [doc = ""] # [doc = " [`Color`]: ratatui_core::style::Color"] # [must_use = "method moves the value of self and returns the modified value"] pub fn style < S : Into < Option < Style > > > (mut self , style : S) -> Self { self . style = style . into () ; self } }
};
}
