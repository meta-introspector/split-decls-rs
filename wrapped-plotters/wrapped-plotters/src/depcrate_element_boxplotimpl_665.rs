// Generated macro for impl_665 (impl)
macro_rules! Depcrate_element_boxplotimpl_665 {
() => {
// Module: crate::element::boxplot
// Provides: {"impl_665"}
// Dependencies: {}
impl < K : Clone > Boxplot < K , BoxplotOrientH < K , f32 > > { # [doc = " Create a new horizontal boxplot element."] # [doc = ""] # [doc = " - `key`: The key (the Y axis value)"] # [doc = " - `quartiles`: The quartiles values for the X axis"] # [doc = " - **returns** The newly created boxplot element"] # [doc = ""] # [doc = " ```rust"] # [doc = " use plotters::prelude::*;"] # [doc = ""] # [doc = " let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);"] # [doc = " let plot = Boxplot::new_horizontal(\"group\", &quartiles);"] # [doc = " ```"] pub fn new_horizontal (key : K , quartiles : & Quartiles) -> Self { Self { style : Into :: < ShapeStyle > :: into (BLACK) , width : DEFAULT_WIDTH , whisker_width : 1.0 , offset : 0.0 , key , values : quartiles . values () , _p : PhantomData , } } }
};
}
