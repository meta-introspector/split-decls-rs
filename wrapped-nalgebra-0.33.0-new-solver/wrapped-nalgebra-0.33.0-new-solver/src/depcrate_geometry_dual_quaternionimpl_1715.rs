// Generated macro for impl_1715 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1715 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1715"}
// Dependencies: {}
impl < T : RealField + fmt :: Display > fmt :: Display for UnitDualQuaternion < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (axis) = self . rotation () . axis () { let axis = axis . into_inner () ; write ! (f , "UnitDualQuaternion translation: {} − angle: {} − axis: ({}, {}, {})" , self . translation () . vector , self . rotation () . angle () , axis [0] , axis [1] , axis [2]) } else { write ! (f , "UnitDualQuaternion translation: {} − angle: {} − axis: (undefined)" , self . translation () . vector , self . rotation () . angle ()) } } }
};
}
