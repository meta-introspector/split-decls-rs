// Generated macro for impl_160 (impl)
macro_rules! Depcrate_plots_colorsimpl_160 {
() => {
// Module: crate::plots::colors
// Provides: {"impl_160"}
// Dependencies: {}
impl ColorCycle { pub fn next_color (& mut self) -> RGBColor { let color = self . colors [self . tracking_index] ; if self . tracking_index == self . colors . len () - 1 { self . tracking_index = 0 } else { self . tracking_index += 1 } color } pub fn reset (& mut self) { self . tracking_index = self . initial_index ; } }
};
}
