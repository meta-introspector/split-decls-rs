// Generated macro for macro_1306 (macro)
macro_rules! Depcrate_geometry_point_constructionmacro_1306 {
() => {
// Module: crate::geometry::point_construction
// Provides: {"macro_1306"}
// Dependencies: {}
componentwise_constructors_impl ! ("# use nalgebra::Point2;\nlet p = Point2::new(1.0, 2.0);\nassert!(p.x == 1.0 && p.y == 2.0);" ; Point2 , Vector2 , x : 0 , y : 1 ; "# use nalgebra::Point3;\nlet p = Point3::new(1.0, 2.0, 3.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0);" ; Point3 , Vector3 , x : 0 , y : 1 , z : 2 ; "# use nalgebra::Point4;\nlet p = Point4::new(1.0, 2.0, 3.0, 4.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0);" ; Point4 , Vector4 , x : 0 , y : 1 , z : 2 , w : 3 ; "# use nalgebra::Point5;\nlet p = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0);" ; Point5 , Vector5 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 ; "# use nalgebra::Point6;\nlet p = Point6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0 && p.b == 6.0);" ; Point6 , Vector6 , x : 0 , y : 1 , z : 2 , w : 3 , a : 4 , b : 5 ;) ;
};
}
