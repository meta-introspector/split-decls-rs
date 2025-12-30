// Generated macro for impl_46 (impl)
macro_rules! Depcrate_helperimpl_46 {
() => {
// Module: crate::helper
// Provides: {"impl_46"}
// Dependencies: {}
impl fmt :: Display for ContainerKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Struct => write ! (f , "struct") , Self :: Union => write ! (f , "union") , } } }
};
}
