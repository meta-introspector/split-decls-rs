// Generated macro for impl_405 (impl)
macro_rules! Depcrate_infer_relate_latticeimpl_405 {
() => {
// Module: crate::infer::relate::lattice
// Provides: {"impl_405"}
// Dependencies: {}
impl LatticeOpKind { fn invert (self) -> Self { match self { LatticeOpKind :: Glb => LatticeOpKind :: Lub , LatticeOpKind :: Lub => LatticeOpKind :: Glb , } } }
};
}
