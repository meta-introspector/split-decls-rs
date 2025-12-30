// Generated macro for impl_1530 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1530 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1530"}
// Dependencies: {}
impl < T : RealField + fmt :: Display > fmt :: Display for Quaternion < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Quaternion {} − ({}, {}, {})" , self [3] , self [0] , self [1] , self [2]) } }
};
}
