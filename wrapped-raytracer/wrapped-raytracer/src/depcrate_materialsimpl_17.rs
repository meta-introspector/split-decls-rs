// Generated macro for impl_17 (impl)
macro_rules! Depcrate_materialsimpl_17 {
() => {
// Module: crate::materials
// Provides: {"impl_17"}
// Dependencies: {}
impl Material for Lambertian { fn scatter (& self , _r_in : & Ray , hit : & Hit) -> Scatter { let target = hit . p + hit . normal + random_in_unit_sphere () ; Scatter { color : self . albedo , ray : Some (Ray :: new (hit . p , target - hit . p)) , } } }
};
}
