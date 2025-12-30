// Generated macro for impl_1537 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1537 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1537"}
// Dependencies: {}
impl < T : RealField + fmt :: Display > fmt :: Display for UnitQuaternion < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (axis) = self . axis () { let axis = axis . into_inner () ; write ! (f , "UnitQuaternion angle: {} − axis: ({}, {}, {})" , self . angle () , axis [0] , axis [1] , axis [2]) } else { write ! (f , "UnitQuaternion angle: {} − axis: (undefined)" , self . angle ()) } } }
};
}
