// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl SelectedTab { fn render_examples (area : Rect , buf : & mut Buffer , flex : Flex , spacing : u16) { let heights = EXAMPLE_DATA . iter () . map (| (desc , _) | get_description_height (desc) + 4) ; let areas = Layout :: vertical (heights) . flex (Flex :: Start) . split (area) ; for (area , (description , constraints)) in areas . iter () . zip (EXAMPLE_DATA . iter ()) { Example :: new (constraints , description , flex , spacing) . render (* area , buf) ; } } }
};
}
