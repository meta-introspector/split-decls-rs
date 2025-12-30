// Generated macro for Boxplot (struct)
macro_rules! Depcrate_element_boxplotBoxplot {
() => {
// Module: crate::element::boxplot
// Provides: {"Boxplot"}
// Dependencies: {}
# [doc = " The boxplot element"] pub struct Boxplot < K , O : BoxplotOrient < K , f32 > > { style : ShapeStyle , width : u32 , whisker_width : f64 , offset : f64 , key : K , values : [f32 ; 5] , _p : PhantomData < O > , }
};
}
