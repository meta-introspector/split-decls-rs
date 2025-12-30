// Generated macro for impl_81 (impl)
macro_rules! Depcrate_chart_meshimpl_81 {
() => {
// Module: crate::chart::mesh
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , 'b , X , Y , XT , YT , DB > MeshStyle < 'a , 'b , X , Y , DB > where X : Ranged < ValueType = XT > + ValueFormatter < XT > , Y : Ranged < ValueType = YT > + ValueFormatter < YT > , DB : DrawingBackend , { pub (crate) fn new (chart : & 'b mut ChartContext < 'a , DB , Cartesian2d < X , Y > >) -> Self { let base_tick_size = (5u32) . percent () . max (5) . in_pixels (chart . plotting_area ()) ; let mut x_tick_size = [base_tick_size , base_tick_size] ; let mut y_tick_size = [base_tick_size , base_tick_size] ; for idx in 0 .. 2 { if chart . is_overlapping_drawing_area (chart . x_label_area [idx] . as_ref ()) { x_tick_size [idx] = - x_tick_size [idx] ; } if chart . is_overlapping_drawing_area (chart . y_label_area [idx] . as_ref ()) { y_tick_size [idx] = - y_tick_size [idx] ; } } MeshStyle { parent_size : chart . drawing_area . dim_in_pixel () , axis_style : None , x_label_offset : 0 , y_label_offset : 0 , draw_x_mesh : true , draw_y_mesh : true , draw_x_axis : true , draw_y_axis : true , x_light_lines_limit : 10 , y_light_lines_limit : 10 , n_x_labels : 11 , n_y_labels : 11 , bold_line_style : None , light_line_style : None , x_label_style : None , y_label_style : None , format_x : None , format_y : None , target : Some (chart) , _phantom_data : PhantomData , x_desc : None , y_desc : None , axis_desc_style : None , x_tick_size , y_tick_size , } } }
};
}
