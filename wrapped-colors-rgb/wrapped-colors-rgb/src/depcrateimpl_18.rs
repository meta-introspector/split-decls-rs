// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " Implement the Widget trait for &mut App so that it can be rendered"] # [doc = ""] # [doc = " This is implemented on a mutable reference so that the app can update its state while it is"] # [doc = " being rendered. This allows the fps widget to update the fps calculation and the colors widget"] # [doc = " to update the colors to render."] impl Widget for & mut App { fn render (self , area : Rect , buf : & mut Buffer) { use Constraint :: { Length , Min } ; let [top , colors] = area . layout (& Layout :: vertical ([Length (1) , Min (0)])) ; let [title , fps] = top . layout (& Layout :: horizontal ([Min (0) , Length (8)])) ; Text :: from ("colors_rgb example. Press q to quit") . centered () . render (title , buf) ; self . fps_widget . render (fps , buf) ; self . colors_widget . render (colors , buf) ; } }
};
}
