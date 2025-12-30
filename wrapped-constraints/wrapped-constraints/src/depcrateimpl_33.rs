// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl Widget for Example { fn render (self , area : Rect , buf : & mut Buffer) { let vertical = Layout :: vertical ([Length (ILLUSTRATION_HEIGHT) , Length (SPACER_HEIGHT)]) ; let horizontal = Layout :: horizontal (& self . constraints) ; let [area , _] = area . layout (& vertical) ; let blocks = area . layout_vec (& horizontal) ; for (block , constraint) in blocks . iter () . zip (& self . constraints) { Self :: illustration (* constraint , block . width) . render (* block , buf) ; } } }
};
}
