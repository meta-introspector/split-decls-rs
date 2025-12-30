// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Widget for & StringField { fn render (self , area : Rect , buf : & mut Buffer) { let layout = Layout :: horizontal ([Constraint :: Length (self . label . len () as u16 + 2) , Constraint :: Fill (1) ,]) ; let [label_area , value_area] = area . layout (& layout) ; let label = Line :: from_iter ([self . label , ": "]) . bold () ; label . render (label_area , buf) ; self . value . clone () . render (value_area , buf) ; } }
};
}
