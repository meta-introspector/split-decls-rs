// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl SelectedTab { # [doc = " Get the previous tab, if there is no previous tab return the current tab."] fn previous (self) -> Self { let current_index : usize = self as usize ; let previous_index = current_index . saturating_sub (1) ; Self :: from_repr (previous_index) . unwrap_or (self) } # [doc = " Get the next tab, if there is no next tab return the current tab."] fn next (self) -> Self { let current_index = self as usize ; let next_index = current_index . saturating_add (1) ; Self :: from_repr (next_index) . unwrap_or (self) } # [doc = " Convert a `SelectedTab` into a `Line` to display it by the `Tabs` widget."] fn to_tab_title (value : Self) -> Line < 'static > { use tailwind :: { INDIGO , ORANGE , SKY } ; let text = value . to_string () ; let color = match value { Self :: Legacy => ORANGE . c400 , Self :: Start => SKY . c400 , Self :: Center => SKY . c300 , Self :: End => SKY . c200 , Self :: SpaceEvenly => INDIGO . c400 , Self :: SpaceBetween => INDIGO . c300 , Self :: SpaceAround => INDIGO . c500 , } ; format ! (" {text} ") . fg (color) . bg (Color :: Black) . into () } }
};
}
