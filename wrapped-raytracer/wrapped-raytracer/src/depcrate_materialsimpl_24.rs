// Generated macro for impl_24 (impl)
macro_rules! Depcrate_materialsimpl_24 {
() => {
// Module: crate::materials
// Provides: {"impl_24"}
// Dependencies: {}
impl Material for Dielectric { fn scatter (& self , r_in : & Ray , hit : & Hit) -> Scatter { let outward_normal : Vec3 ; let ni_over_nt : f32 ; if r_in . direction . dot (hit . normal) > 0.0 { outward_normal = - hit . normal ; ni_over_nt = self . index ; } else { outward_normal = hit . normal ; ni_over_nt = 1.0 / self . index ; } let _ = refract (r_in . direction , outward_normal , ni_over_nt) ; Scatter { color : WHITE , ray : Some (Ray :: new (hit . p , reflect (r_in . direction , hit . normal))) , } } }
};
}
