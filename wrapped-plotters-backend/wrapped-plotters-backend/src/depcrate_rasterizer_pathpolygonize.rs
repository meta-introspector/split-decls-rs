// Generated macro for polygonize (function)
macro_rules! Depcrate_rasterizer_pathpolygonize {
() => {
// Module: crate::rasterizer::path
// Provides: {"polygonize"}
// Dependencies: {}
# [doc = " Covert a path with >1px stroke width into polygon."] pub fn polygonize (vertices : & [BackendCoord] , stroke_width : u32) -> Vec < BackendCoord > { if vertices . len () < 2 { return vec ! [] ; } let mut ret = vec ! [] ; traverse_vertices (vertices . iter () , stroke_width , | v | ret . push (v)) ; traverse_vertices (vertices . iter () . rev () , stroke_width , | v | ret . push (v)) ; ret }
};
}
