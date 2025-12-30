// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
# [doc = " `RedSquare` and `BlueSquare` are widgets that render a red and blue square, respectively. They"] # [doc = " implement the `WidgetRef` trait instead of the `Widget` trait, which which allows them to be"] # [doc = " rendered as boxed widgets. It's not possible to use Widget for this as a dynamic reference to a"] # [doc = " widget cannot generally be moved out of the box."] impl WidgetRef for RedSquare { fn render_ref (& self , area : Rect , buf : & mut Buffer) { fill (area , buf , "█" , Color :: Red) ; } }
};
}
