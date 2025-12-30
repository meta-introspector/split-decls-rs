// Generated macro for MeshStyle (struct)
macro_rules! Depcrate_chart_meshMeshStyle {
() => {
// Module: crate::chart::mesh
// Provides: {"MeshStyle"}
// Dependencies: {}
# [doc = " The struct that is used for tracking the configuration of a mesh of any chart"] pub struct MeshStyle < 'a , 'b , X : Ranged , Y : Ranged , DB : DrawingBackend > { pub (super) parent_size : (u32 , u32) , pub (super) draw_x_mesh : bool , pub (super) draw_y_mesh : bool , pub (super) draw_x_axis : bool , pub (super) draw_y_axis : bool , pub (super) x_label_offset : i32 , pub (super) y_label_offset : i32 , pub (super) x_light_lines_limit : usize , pub (super) y_light_lines_limit : usize , pub (super) n_x_labels : usize , pub (super) n_y_labels : usize , pub (super) axis_desc_style : Option < TextStyle < 'b > > , pub (super) x_desc : Option < String > , pub (super) y_desc : Option < String > , pub (super) bold_line_style : Option < ShapeStyle > , pub (super) light_line_style : Option < ShapeStyle > , pub (super) axis_style : Option < ShapeStyle > , pub (super) x_label_style : Option < TextStyle < 'b > > , pub (super) y_label_style : Option < TextStyle < 'b > > , pub (super) format_x : Option < & 'b dyn Fn (& X :: ValueType) -> String > , pub (super) format_y : Option < & 'b dyn Fn (& Y :: ValueType) -> String > , pub (super) target : Option < & 'b mut ChartContext < 'a , DB , Cartesian2d < X , Y > > > , pub (super) _phantom_data : PhantomData < (X , Y) > , pub (super) x_tick_size : [i32 ; 2] , pub (super) y_tick_size : [i32 ; 2] , }
};
}
