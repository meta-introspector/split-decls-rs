// Generated macro for macro_2044 (macro)
macro_rules! Depcrate_geometry_translation_constructionmacro_2044 {
() => {
// Module: crate::geometry::translation_construction
// Provides: {"macro_2044"}
// Dependencies: {}
componentwise_constructors_impl ! ("# use nalgebra::Translation1;\nlet t = Translation1::new(1.0);\nassert!(t.vector.x == 1.0);" ; 1 , x : 0 ; "# use nalgebra::Translation2;\nlet t = Translation2::new(1.0, 2.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0);" ; 2 , x : 0 , y : 1 ; "# use nalgebra::Translation3;\nlet t = Translation3::new(1.0, 2.0, 3.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0);" ; 3 , x : 0 , y : 1 , z : 2 ; "# use nalgebra::Translation4;\nlet t = Translation4::new(1.0, 2.0, 3.0, 4.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0);" ; 4 , x : 0 , y : 1 , z : 2 , w : 3 ; "# use nalgebra::Translation5;\nlet t = Translation5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0);" ; 5 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 ; "# use nalgebra::Translation6;\nlet t = Translation6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0 && t.vector.b == 6.0);" ; 6 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 , b : 5 ;) ;
};
}
