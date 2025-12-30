// Generated macro for impl_481 (impl)
macro_rules! Depcrate_drawing_areaimpl_481 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_481"}
// Dependencies: {}
impl < DB : DrawingBackend , X : Ranged , Y : Ranged > DrawingArea < DB , Cartesian2d < X , Y > > { # [doc = " Draw the mesh on a area"] pub fn draw_mesh < DrawFunc , YH : KeyPointHint , XH : KeyPointHint > (& self , mut draw_func : DrawFunc , y_count_max : YH , x_count_max : XH ,) -> Result < () , DrawingAreaErrorKind < DB :: ErrorType > > where DrawFunc : FnMut (& mut DB , MeshLine < X , Y >) -> Result < () , DrawingErrorKind < DB :: ErrorType > > , { self . backend_ops (move | b | { self . coord . draw_mesh (y_count_max , x_count_max , | line | draw_func (b , line)) }) } # [doc = " Get the range of X of the guest coordinate for current drawing area"] pub fn get_x_range (& self) -> Range < X :: ValueType > { self . coord . get_x_range () } # [doc = " Get the range of Y of the guest coordinate for current drawing area"] pub fn get_y_range (& self) -> Range < Y :: ValueType > { self . coord . get_y_range () } # [doc = " Get the range of X of the backend coordinate for current drawing area"] pub fn get_x_axis_pixel_range (& self) -> Range < i32 > { self . coord . get_x_axis_pixel_range () } # [doc = " Get the range of Y of the backend coordinate for current drawing area"] pub fn get_y_axis_pixel_range (& self) -> Range < i32 > { self . coord . get_y_axis_pixel_range () } }
};
}
