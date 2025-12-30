// Generated macro for impl_563 (impl)
macro_rules! Depcrate_element_basic_shapes_3dimpl_563 {
() => {
// Module: crate::element::basic_shapes_3d
// Provides: {"impl_563"}
// Dependencies: {}
impl < X , Y , Z , DB : DrawingBackend > Drawable < DB , BackendCoordAndZ > for Cubiod < X , Y , Z > { fn draw < I : Iterator < Item = (BackendCoord , i32) > > (& self , points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let vert : Vec < _ > = points . collect () ; let mut polygon = vec ! [] ; for mask in [1 , 2 , 4] . iter () . cloned () { let mask_a = if mask == 4 { 1 } else { mask * 2 } ; let mask_b = if mask == 1 { 4 } else { mask / 2 } ; let a = 0 ; let b = a | mask_a ; let c = a | mask_a | mask_b ; let d = a | mask_b ; polygon . push ([vert [a] , vert [b] , vert [c] , vert [d]]) ; polygon . push ([vert [a | mask] , vert [b | mask] , vert [c | mask] , vert [d | mask] ,]) ; } polygon . sort_by_cached_key (| t | std :: cmp :: Reverse (t [0] . 1 + t [1] . 1 + t [2] . 1 + t [3] . 1)) ; for p in polygon { backend . fill_polygon (p . iter () . map (| (coord , _) | * coord) , & self . face_style) ? ; backend . draw_path (p . iter () . map (| (coord , _) | * coord) . chain (std :: iter :: once (p [0] . 0)) , & self . edge_style ,) ? ; } Ok (()) } }
};
}
