// Generated macro for impl_338 (impl)
macro_rules! Depcrate_tabsimpl_338 {
() => {
// Module: crate::tabs
// Provides: {"impl_338"}
// Dependencies: {}
impl Default for Tabs < '_ > { # [doc = " Returns a default `Tabs` widget."] # [doc = ""] # [doc = " The default widget has:"] # [doc = " - No tabs"] # [doc = " - No selected tab"] # [doc = " - The highlight style is set to reversed."] # [doc = " - The divider is set to a pipe (`|`)."] # [doc = " - The padding on the left and right is set to a space."] # [doc = ""] # [doc = " This is rarely useful on its own without calling [`Tabs::titles`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::Tabs;"] # [doc = ""] # [doc = " let tabs = Tabs::default().titles([\"Tab 1\", \"Tab 2\"]);"] # [doc = " ```"] fn default () -> Self { Self :: new (Vec :: < Line > :: new ()) } }
};
}
