// Generated macro for impl_343 (impl)
macro_rules! Depcrate_tabsimpl_343 {
() => {
// Module: crate::tabs
// Provides: {"impl_343"}
// Dependencies: {}
impl Tabs < '_ > { fn render_tabs (& self , tabs_area : Rect , buf : & mut Buffer) { if tabs_area . is_empty () { return ; } let mut x = tabs_area . left () ; let titles_length = self . titles . len () ; for (i , title) in self . titles . iter () . enumerate () { let last_title = titles_length - 1 == i ; let remaining_width = tabs_area . right () . saturating_sub (x) ; if remaining_width == 0 { break ; } let pos = buf . set_line (x , tabs_area . top () , & self . padding_left , remaining_width) ; x = pos . 0 ; let remaining_width = tabs_area . right () . saturating_sub (x) ; if remaining_width == 0 { break ; } let pos = buf . set_line (x , tabs_area . top () , title , remaining_width) ; if Some (i) == self . selected { buf . set_style (Rect { x , y : tabs_area . top () , width : pos . 0 . saturating_sub (x) , height : 1 , } , self . highlight_style ,) ; } x = pos . 0 ; let remaining_width = tabs_area . right () . saturating_sub (x) ; if remaining_width == 0 { break ; } let pos = buf . set_line (x , tabs_area . top () , & self . padding_right , remaining_width) ; x = pos . 0 ; let remaining_width = tabs_area . right () . saturating_sub (x) ; if remaining_width == 0 || last_title { break ; } let pos = buf . set_span (x , tabs_area . top () , & self . divider , remaining_width) ; x = pos . 0 ; } } }
};
}
