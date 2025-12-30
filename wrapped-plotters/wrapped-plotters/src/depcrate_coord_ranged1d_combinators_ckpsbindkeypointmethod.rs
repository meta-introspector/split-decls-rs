// Generated macro for BindKeyPointMethod (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsBindKeyPointMethod {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"BindKeyPointMethod"}
// Dependencies: {}
# [doc = " Bind an existing coordinate spec with a given key points algorithm. See [WithKeyPointMethod](struct.WithKeyMethod.html ) for more details."] pub trait BindKeyPointMethod where Self : AsRangedCoord , { # [doc = " Bind a existing coordinate spec with a given key points algorithm. See [WithKeyPointMethod](struct.WithKeyMethod.html ) for more details."] # [doc = " Example:"] # [doc = " ```"] # [doc = "use plotters::prelude::*;"] # [doc = "use plotters_bitmap::BitMapBackend;"] # [doc = "let mut buffer = vec![0;1024*768*3];"] # [doc = " let root = BitMapBackend::with_buffer(&mut buffer, (1024, 768)).into_drawing_area();"] # [doc = " let mut chart = ChartBuilder::on(&root)"] # [doc = "    .build_cartesian_2d("] # [doc = "        (0..100).with_key_point_func(|n| (0..100 / n as i32).map(|x| x * 100 / n as i32).collect()),"] # [doc = "        0..10"] # [doc = " ).unwrap();"] # [doc = " chart.configure_mesh().draw().unwrap();"] # [doc = "```"] fn with_key_point_func < F : Fn (usize) -> Vec < Self :: Value > + 'static > (self , func : F ,) -> WithKeyPointMethod < Self :: CoordDescType > { WithKeyPointMethod { inner : self . into () , bold_func : Box :: new (func) , light_func : Box :: new (| _ | Vec :: new ()) , } } }
};
}
