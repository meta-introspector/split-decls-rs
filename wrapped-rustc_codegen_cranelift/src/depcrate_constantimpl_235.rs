// Generated macro for impl_235 (impl)
macro_rules! Depcrate_constantimpl_235 {
() => {
// Module: crate::constant
// Provides: {"impl_235"}
// Dependencies: {}
impl ConstantCx { pub (crate) fn new () -> Self { ConstantCx { todo : vec ! [] , anon_allocs : FxHashMap :: default () } } pub (crate) fn finalize (mut self , tcx : TyCtxt < '_ > , module : & mut dyn Module) { define_all_allocs (tcx , module , & mut self) ; } }
};
}
