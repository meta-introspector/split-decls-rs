// Generated macro for impl_55 (impl)
macro_rules! Depcrate_vecimpl_55 {
() => {
// Module: crate::vec
// Provides: {"impl_55"}
// Dependencies: {}
impl Ray { pub fn new (o : Vec3 , d : Vec3) -> Ray { Ray { origin : o , direction : d , } } pub fn point_at_parameter (& self , t : f32) -> Vec3 { self . origin + t * self . direction } }
};
}
