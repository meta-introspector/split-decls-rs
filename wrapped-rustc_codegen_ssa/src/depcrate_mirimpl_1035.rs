// Generated macro for impl_1035 (impl)
macro_rules! Depcrate_mirimpl_1035 {
() => {
// Module: crate::mir
// Provides: {"impl_1035"}
// Dependencies: {}
impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub fn monomorphize < T > (& self , value : T) -> T where T : Copy + TypeFoldable < TyCtxt < 'tcx > > , { debug ! ("monomorphize: self.instance={:?}" , self . instance) ; self . instance . instantiate_mir_and_normalize_erasing_regions (self . cx . tcx () , self . cx . typing_env () , ty :: EarlyBinder :: bind (value) ,) } }
};
}
