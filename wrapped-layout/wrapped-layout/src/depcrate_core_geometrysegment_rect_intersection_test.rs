// Generated macro for segment_rect_intersection_test (function)
macro_rules! Depcrate_core_geometrysegment_rect_intersection_test {
() => {
// Module: crate::core::geometry
// Provides: {"segment_rect_intersection_test"}
// Dependencies: {}
# [test] fn segment_rect_intersection_test () { let v0 = (Point :: new (- 48. , - 27.) , Point :: new (- 196. , - 55.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; let v1 = (Point :: new (- 70. , - 156.) , Point :: new (57. , 41.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; let v2 = (Point :: new (70. , - 11.) , Point :: new (- 20. , - 119.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; assert ! (segment_rect_intersection ((v0 . 0 , v0 . 1) , (v0 . 2 , v0 . 3))) ; assert ! (segment_rect_intersection ((v1 . 0 , v1 . 1) , (v1 . 2 , v1 . 3))) ; assert ! (segment_rect_intersection ((v2 . 0 , v2 . 1) , (v2 . 2 , v2 . 3))) ; let v0 = (Point :: new (190. , - 55.) , Point :: new (173. , 199.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; let v1 = (Point :: new (142. , - 19.) , Point :: new (- 108. , - 133.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; let v2 = (Point :: new (151. , 80.) , Point :: new (17. , 124.) , Point :: new (- 50. , - 50.) , Point :: new (50. , 50.) ,) ; assert ! (! segment_rect_intersection ((v0 . 0 , v0 . 1) , (v0 . 2 , v0 . 3))) ; assert ! (! segment_rect_intersection ((v1 . 0 , v1 . 1) , (v1 . 2 , v1 . 3))) ; assert ! (! segment_rect_intersection ((v2 . 0 , v2 . 1) , (v2 . 2 , v2 . 3))) ; }
};
}
