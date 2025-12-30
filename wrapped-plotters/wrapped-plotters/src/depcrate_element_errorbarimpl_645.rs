// Generated macro for impl_645 (impl)
macro_rules! Depcrate_element_errorbarimpl_645 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_645"}
// Dependencies: {}
impl < K , V > ErrorBar < K , V , ErrorBarOrientV < K , V > > { # [doc = "\n    Creates a vertical error bar.\n    `\n    - `key`: Horizontal position of the bar\n    - `min`: Minimum of the data\n    - `avg`: Average of the data\n    - `max`: Maximum of the data\n    - `style`: Color, transparency, and fill of the error bar. See [`ShapeStyle`] for more information and examples.\n    - `width`: Width of the error marks in backend coordinates.\n\n    See [`ErrorBar`] for more information and examples.\n    "] pub fn new_vertical < S : Into < ShapeStyle > > (key : K , min : V , avg : V , max : V , style : S , width : u32 ,) -> Self { Self { style : style . into () , width , key , values : [min , avg , max] , _p : PhantomData , } } }
};
}
