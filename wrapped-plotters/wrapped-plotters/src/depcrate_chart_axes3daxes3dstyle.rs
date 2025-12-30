// Generated macro for Axes3dStyle (struct)
macro_rules! Depcrate_chart_axes3dAxes3dStyle {
() => {
// Module: crate::chart::axes3d
// Provides: {"Axes3dStyle"}
// Dependencies: {}
# [doc = "\nImplements 3D plot axes configurations.\n\nThe best way to use this struct is by way of the [`configure_axes()`] function.\nSee [`ChartContext::configure_axes()`] for more information and examples.\n"] pub struct Axes3dStyle < 'a , 'b , X : Ranged , Y : Ranged , Z : Ranged , DB : DrawingBackend > { pub (super) parent_size : (u32 , u32) , pub (super) target : Option < & 'b mut ChartContext < 'a , DB , Cartesian3d < X , Y , Z > > > , pub (super) tick_size : i32 , pub (super) light_lines_limit : [usize ; 3] , pub (super) n_labels : [usize ; 3] , pub (super) bold_line_style : ShapeStyle , pub (super) light_line_style : ShapeStyle , pub (super) axis_panel_style : ShapeStyle , pub (super) axis_style : ShapeStyle , pub (super) label_style : TextStyle < 'b > , pub (super) format_x : & 'b dyn Fn (& X :: ValueType) -> String , pub (super) format_y : & 'b dyn Fn (& Y :: ValueType) -> String , pub (super) format_z : & 'b dyn Fn (& Z :: ValueType) -> String , _phantom : PhantomData < & 'a (X , Y , Z) > , }
};
}
