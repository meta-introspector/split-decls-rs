// Generated macro for impl_795 (impl)
macro_rules! Depcrate_series_surfaceimpl_795 {
() => {
// Module: crate::series::surface
// Provides: {"impl_795"}
// Dependencies: {}
impl < 'a , X , Y , Z , D , SurfaceFunc > Iterator for SurfaceSeries < 'a , X , Y , Z , D , SurfaceFunc > where D : Direction < X , Y , Z > , D :: Input1Type : Clone , D :: Input2Type : Clone , SurfaceFunc : Fn (D :: Input1Type , D :: Input2Type) -> D :: OutputType , { type Item = Polygon < (X , Y , Z) > ; fn next (& mut self) -> Option < Self :: Item > { let (b0 , b1) = if let (Some (b0) , Some (b1)) = (self . free_var_2 . get (self . vidx_2) , self . free_var_2 . get (self . vidx_2 + 1) ,) { self . vidx_2 += 1 ; (b0 , b1) } else { self . vidx_1 += 1 ; self . vidx_2 = 1 ; if let (Some (b0) , Some (b1)) = (self . free_var_2 . first () , self . free_var_2 . get (1)) { (b0 , b1) } else { return None ; } } ; match (self . free_var_1 . get (self . vidx_1) , self . free_var_1 . get (self . vidx_1 + 1) ,) { (Some (a0) , Some (a1)) => { let value = (self . surface_f) (a0 . clone () , b0 . clone ()) ; let style = self . style . get_style (& value) ; let vert = vec ! [D :: make_coord ((a0 . clone () , b0 . clone ()) , value) , D :: make_coord ((a0 . clone () , b1 . clone ()) , (self . surface_f) (a0 . clone () , b1 . clone ()) ,) , D :: make_coord ((a1 . clone () , b1 . clone ()) , (self . surface_f) (a1 . clone () , b1 . clone ()) ,) , D :: make_coord ((a1 . clone () , b0 . clone ()) , (self . surface_f) (a1 . clone () , b0 . clone ()) ,) ,] ; Some (Polygon :: new (vert , style)) } _ => None , } } }
};
}
