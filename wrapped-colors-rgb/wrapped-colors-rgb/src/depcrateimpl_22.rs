// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Widget impl for `ColorsWidget`"] # [doc = ""] # [doc = " This is implemented on a mutable reference so that we can update the frame count and store a"] # [doc = " cached version of the colors to render instead of recalculating them every frame."] impl Widget for & mut ColorsWidget { # [doc = " Render the widget"] fn render (self , area : Rect , buf : & mut Buffer) { self . setup_colors (area) ; let colors = & self . colors ; for (xi , x) in (area . left () .. area . right ()) . enumerate () { let xi = (xi + self . frame_count) % (area . width as usize) ; for (yi , y) in (area . top () .. area . bottom ()) . enumerate () { let fg = colors [yi * 2] [xi] ; let bg = colors [yi * 2 + 1] [xi] ; buf [Position :: new (x , y)] . set_char ('▀') . set_fg (fg) . set_bg (bg) ; } } self . frame_count += 1 ; } }
};
}
