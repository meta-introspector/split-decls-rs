// Generated macro for RenderBackend (trait)
macro_rules! Depcrate_core_formatRenderBackend {
() => {
// Module: crate::core::format
// Provides: {"RenderBackend"}
// Dependencies: {}
# [doc = " This is the trait that all rendering backends need to implement."] pub trait RenderBackend { # [doc = " Draw a rectangle. The top-left point of the rectangle is \\p xy. The shape"] # [doc = " style (color, edge-width) are passed in \\p look. The parameter \\p clip"] # [doc = " is an optional clip region (see: create_clip)."] fn draw_rect (& mut self , xy : Point , size : Point , look : & StyleAttr , properties : Option < String > , clip : Option < ClipHandle > ,) ; # [doc = " Draw a line between \\p start and \\p stop."] fn draw_line (& mut self , start : Point , stop : Point , look : & StyleAttr , properties : Option < String > ,) ; # [doc = " Draw an ellipse with the center \\p xy, and size \\p size."] fn draw_circle (& mut self , xy : Point , size : Point , look : & StyleAttr , properties : Option < String > ,) ; # [doc = " Draw a labe."] fn draw_text (& mut self , xy : Point , text : & str , look : & StyleAttr) ; # [doc = " Draw an arrow, with a label, with the style parameters in \\p look."] fn draw_arrow (& mut self , path : & [(Point , Point)] , dashed : bool , head : (bool , bool) , look : & StyleAttr , properties : Option < String > , text : & str ,) ; # [doc = " Generate a clip region that shapes can use to create complex shapes."] fn create_clip (& mut self , xy : Point , size : Point , rounded_px : usize ,) -> ClipHandle ; }
};
}
