// Generated macro for impl_223 (impl)
macro_rules! Depcrate_builderimpl_223 {
() => {
// Module: crate::builder
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a , 'gcc , 'tcx > StaticBuilderMethods for Builder < 'a , 'gcc , 'tcx > { fn get_static (& mut self , def_id : DefId) -> RValue < 'gcc > { self . cx () . get_static (def_id) . get_address (self . location) } }
};
}
