// Generated macro for impl_150 (impl)
macro_rules! Depcrate_check_consts_resolverimpl_150 {
() => {
// Module: crate::check_consts::resolver
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'mir , 'tcx , Q > FlowSensitiveAnalysis < 'mir , 'tcx , Q > where Q : Qualif , { pub (super) fn new (_ : Q , ccx : & 'mir ConstCx < 'mir , 'tcx >) -> Self { FlowSensitiveAnalysis { ccx , _qualif : PhantomData } } fn transfer_function (& self , state : & 'mir mut State) -> TransferFunction < 'mir , 'tcx , Q > { TransferFunction :: < Q > :: new (self . ccx , state) } }
};
}
