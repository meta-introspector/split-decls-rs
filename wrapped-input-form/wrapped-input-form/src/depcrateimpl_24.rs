// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl Widget for & AgeField { fn render (self , area : Rect , buf : & mut Buffer) { let layout = Layout :: horizontal ([Constraint :: Length (self . label . len () as u16 + 2) , Constraint :: Fill (1) ,]) ; let [label_area , value_area] = area . layout (& layout) ; let label = Line :: from_iter ([self . label , ": "]) . bold () ; let value = self . value . to_string () ; label . render (label_area , buf) ; value . render (value_area , buf) ; } }
};
}
