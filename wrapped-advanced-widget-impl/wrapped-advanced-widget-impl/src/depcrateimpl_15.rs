// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl Widget for Greeting { fn render (self , area : Rect , buf : & mut Buffer) { let greeting = format ! ("Hello, {}!" , self . name) ; greeting . render (area , buf) ; } }
};
}
