// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Widget for & Hyperlink < '_ > { fn render (self , area : Rect , buffer : & mut Buffer) { (& self . text) . render (area , buffer) ; for (i , two_chars) in self . text . to_string () . chars () . chunks (2) . into_iter () . enumerate () { let text = two_chars . collect :: < String > () ; let hyperlink = format ! ("\x1B]8;;{}\x07{}\x1B]8;;\x07" , self . url , text) ; buffer [(area . x + i as u16 * 2 , area . y)] . set_symbol (hyperlink . as_str ()) ; } } }
};
}
