// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
# [doc = " This implements the `Widget` trait on a reference to the type. It contains a list of boxed"] # [doc = " widgets that implement the `WidgetRef` trait. This is useful for widgets that contain a list of"] # [doc = " other widgets that can be different types."] impl Widget for & BoxedSquares { fn render (self , area : Rect , buf : & mut Buffer) { let constraints = vec ! [Constraint :: Length (4) ; self . squares . len ()] ; let areas = area . layout_vec (& Layout :: horizontal (constraints)) ; for (widget , area) in self . squares . iter () . zip (areas) { widget . render_ref (area , buf) ; } } }
};
}
