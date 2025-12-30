// Generated macro for impl_20 (impl)
macro_rules! Depcrate_materialsimpl_20 {
() => {
// Module: crate::materials
// Provides: {"impl_20"}
// Dependencies: {}
impl Material for Metal { fn scatter (& self , r_in : & Ray , hit : & Hit) -> Scatter { let reflected = reflect (r_in . direction , hit . normal) ; let scattered = Ray :: new (hit . p , reflected + self . fuzz * random_in_unit_sphere ()) ; Scatter { color : self . albedo , ray : if scattered . direction . dot (hit . normal) <= 0.0 { None } else { Some (scattered) } , } } }
};
}
