// Generated macro for traverse_vertices (function)
macro_rules! Depcrate_rasterizer_pathtraverse_vertices {
() => {
// Module: crate::rasterizer::path
// Provides: {"traverse_vertices"}
// Dependencies: {}
fn traverse_vertices < 'a > (mut vertices : impl Iterator < Item = & 'a BackendCoord > , width : u32 , mut op : impl FnMut (BackendCoord) ,) { let mut a = vertices . next () . unwrap () ; let mut b = vertices . next () . unwrap () ; while a == b { a = b ; if let Some (new_b) = vertices . next () { b = new_b ; } else { return ; } } let (_ , n) = get_dir_vector (* a , * b , false) ; op (((f64 :: from (a . 0) + n . 0 * f64 :: from (width) / 2.0) . round () as i32 , (f64 :: from (a . 1) + n . 1 * f64 :: from (width) / 2.0) . round () as i32 ,)) ; let mut recent = [(0 , 0) , * a , * b] ; let mut vertex_buf = Vec :: with_capacity (3) ; for p in vertices { if * p == recent [2] { continue ; } recent . swap (0 , 1) ; recent . swap (1 , 2) ; recent [2] = * p ; compute_polygon_vertex (& recent , f64 :: from (width) / 2.0 , & mut vertex_buf) ; vertex_buf . iter () . cloned () . for_each (& mut op) ; } let b = recent [1] ; let a = recent [2] ; let (_ , n) = get_dir_vector (a , b , true) ; op (((f64 :: from (a . 0) + n . 0 * f64 :: from (width) / 2.0) . round () as i32 , (f64 :: from (a . 1) + n . 1 * f64 :: from (width) / 2.0) . round () as i32 ,)) ; }
};
}
