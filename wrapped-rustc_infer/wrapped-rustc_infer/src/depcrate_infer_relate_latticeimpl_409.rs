// Generated macro for impl_409 (impl)
macro_rules! Depcrate_infer_relate_latticeimpl_409 {
() => {
// Module: crate::infer::relate::lattice
// Provides: {"impl_409"}
// Dependencies: {}
impl < 'infcx , 'tcx > LatticeOp < 'infcx , 'tcx > { fn relate_bound (& mut self , v : Ty < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , () > { let at = self . infcx . at (& self . trace . cause , self . param_env) ; match self . kind { LatticeOpKind :: Glb => { self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , v , a) ? . into_obligations ()) ; self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , v , b) ? . into_obligations ()) ; } LatticeOpKind :: Lub => { self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , a , v) ? . into_obligations ()) ; self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , b , v) ? . into_obligations ()) ; } } Ok (()) } }
};
}
