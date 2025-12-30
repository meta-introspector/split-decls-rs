// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl SelectedTab { # [doc = " Get the previous tab, if there is no previous tab return the current tab."] fn previous (self) -> Self { let current_index : usize = self as usize ; let previous_index = current_index . saturating_sub (1) ; Self :: from_repr (previous_index) . unwrap_or (self) } # [doc = " Get the next tab, if there is no next tab return the current tab."] fn next (self) -> Self { let current_index = self as usize ; let next_index = current_index . saturating_add (1) ; Self :: from_repr (next_index) . unwrap_or (self) } const fn get_example_count (self) -> u16 { # [expect (clippy :: match_same_arms)] match self { Self :: Length => 4 , Self :: Percentage => 5 , Self :: Ratio => 4 , Self :: Fill => 2 , Self :: Min => 5 , Self :: Max => 5 , } } fn to_tab_title (value : Self) -> Line < 'static > { let text = format ! ("  {value}  ") ; let color = match value { Self :: Length => LENGTH_COLOR , Self :: Percentage => PERCENTAGE_COLOR , Self :: Ratio => RATIO_COLOR , Self :: Fill => FILL_COLOR , Self :: Min => MIN_COLOR , Self :: Max => MAX_COLOR , } ; text . fg (tailwind :: SLATE . c200) . bg (color) . into () } }
};
}
