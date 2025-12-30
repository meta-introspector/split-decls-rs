// Generated macro for Renderable (trait)
macro_rules! Depcrate_core_formatRenderable {
() => {
// Module: crate::core::format
// Provides: {"Renderable"}
// Dependencies: {}
# [doc = " This is the trait that all elements that can be rendered on a canvas need to"] # [doc = " implement."] pub trait Renderable { # [doc = " Render the shape into a canvas."] # [doc = " If \\p debug is set then extra markers will be rendered."] fn render (& self , debug : bool , canvas : & mut dyn RenderBackend) ; # [doc = " \\Return the coordinate for the connection point of an arrow that's"] # [doc = " coming from the direction of \\p from."] # [doc = " The format of the path is (x, y, cx, cy), where cx and cy, are the"] # [doc = " control points of the bezier curve."] # [doc = " \\p force is the magnitude of the edge direction."] # [doc = " \\p port is the optional port name (for named records)."] fn get_connector_location (& self , from : Point , force : f64 , port : & Option < String > ,) -> (Point , Point) ; # [doc = " Computes the coordinate for the connection point of an arrow that's"] # [doc = " passing through this edge."] # [doc = " coming from the direction of \\p from."] # [doc = " \\returns the bezier path in the format (x, y, cx, cy), where cx and cy,"] # [doc = " are the control points for the entry path of the bezier curve. The exit"] # [doc = " path is assumed to be the mirror point for the center (first point)."] # [doc = " \\p force is the magnitude of the edge direction."] # [doc = " This works with the get_connector_location method for drawing edges."] fn get_passthrough_path (& self , from : Point , to : Point , force : f64 ,) -> (Point , Point) ; }
};
}
