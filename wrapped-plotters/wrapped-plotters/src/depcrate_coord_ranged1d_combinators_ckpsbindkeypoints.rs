// Generated macro for BindKeyPoints (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsBindKeyPoints {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"BindKeyPoints"}
// Dependencies: {}
# [doc = " Bind a existing coordinate spec with a given key points vector. See [WithKeyPoints](struct.WithKeyPoints.html ) for more details."] pub trait BindKeyPoints where Self : AsRangedCoord , { # [doc = " Bind a existing coordinate spec with a given key points vector. See [WithKeyPoints](struct.WithKeyPoints.html ) for more details."] # [doc = " Example:"] # [doc = " ```"] # [doc = "use plotters::prelude::*;"] # [doc = "use plotters_bitmap::BitMapBackend;"] # [doc = "let mut buffer = vec![0;1024*768*3];"] # [doc = " let root = BitMapBackend::with_buffer(&mut buffer, (1024, 768)).into_drawing_area();"] # [doc = " let mut chart = ChartBuilder::on(&root)"] # [doc = "    .build_cartesian_2d("] # [doc = "        (0..100).with_key_points(vec![1,20,50,90]),   // <= This line will make the plot shows 4 tick marks at 1, 20, 50, 90"] # [doc = "        0..10"] # [doc = " ).unwrap();"] # [doc = " chart.configure_mesh().draw().unwrap();"] # [doc = "```"] fn with_key_points (self , points : Vec < Self :: Value >) -> WithKeyPoints < Self :: CoordDescType > { WithKeyPoints { inner : self . into () , bold_points : points , light_points : vec ! [] , } } }
};
}
