// Generated macro for macro_2161 (macro)
macro_rules! Depcrate_geometry_scale_constructionmacro_2161 {
() => {
// Module: crate::geometry::scale_construction
// Provides: {"macro_2161"}
// Dependencies: {}
componentwise_constructors_impl ! ("# use nalgebra::Scale1;\nlet t = Scale1::new(1.0);\nassert!(t.vector.x == 1.0);" ; 1 , x : 0 ; "# use nalgebra::Scale2;\nlet t = Scale2::new(1.0, 2.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0);" ; 2 , x : 0 , y : 1 ; "# use nalgebra::Scale3;\nlet t = Scale3::new(1.0, 2.0, 3.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0);" ; 3 , x : 0 , y : 1 , z : 2 ; "# use nalgebra::Scale4;\nlet t = Scale4::new(1.0, 2.0, 3.0, 4.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0);" ; 4 , x : 0 , y : 1 , z : 2 , w : 3 ; "# use nalgebra::Scale5;\nlet t = Scale5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0);" ; 5 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 ; "# use nalgebra::Scale6;\nlet t = Scale6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0 && t.vector.b == 6.0);" ; 6 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 , b : 5 ;) ;
};
}
