// Generated macro for impl_646 (impl)
macro_rules! Depcrate_element_errorbarimpl_646 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_646"}
// Dependencies: {}
impl < K , V > ErrorBar < K , V , ErrorBarOrientH < K , V > > { # [doc = "\n    Creates a horizontal error bar.\n\n    - `key`: Vertical position of the bar\n    - `min`: Minimum of the data\n    - `avg`: Average of the data\n    - `max`: Maximum of the data\n    - `style`: Color, transparency, and fill of the error bar. See [`ShapeStyle`] for more information and examples.\n    - `width`: Width of the error marks in backend coordinates.\n\n    See [`ErrorBar`] for more information and examples.\n    "] pub fn new_horizontal < S : Into < ShapeStyle > > (key : K , min : V , avg : V , max : V , style : S , width : u32 ,) -> Self { Self { style : style . into () , width , key , values : [min , avg , max] , _p : PhantomData , } } }
};
}
