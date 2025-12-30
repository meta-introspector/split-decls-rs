// Generated macro for impl_266 (impl)
macro_rules! Depcrate_scrollbarimpl_266 {
() => {
// Module: crate::scrollbar
// Provides: {"impl_266"}
// Dependencies: {}
impl ScrollbarOrientation { # [doc = " Returns `true` if the scrollbar is vertical."] # [must_use = "returns the requested kind of the scrollbar"] pub const fn is_vertical (& self) -> bool { matches ! (self , Self :: VerticalRight | Self :: VerticalLeft) } # [doc = " Returns `true` if the scrollbar is horizontal."] # [must_use = "returns the requested kind of the scrollbar"] pub const fn is_horizontal (& self) -> bool { matches ! (self , Self :: HorizontalBottom | Self :: HorizontalTop) } }
};
}
