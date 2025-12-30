// Generated macro for impl_1532 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1532 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1532"}
// Dependencies: {}
impl < T : Scalar + ClosedNeg + PartialEq > PartialEq for UnitQuaternion < T > { # [inline] fn eq (& self , rhs : & Self) -> bool { self . coords == rhs . coords || self . coords . iter () . zip (rhs . coords . iter ()) . all (| (a , b) | * a == - b . clone ()) } }
};
}
