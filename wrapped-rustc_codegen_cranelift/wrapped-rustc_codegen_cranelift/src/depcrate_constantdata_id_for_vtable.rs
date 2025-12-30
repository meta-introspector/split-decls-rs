// Generated macro for data_id_for_vtable (function)
macro_rules! Depcrate_constantdata_id_for_vtable {
() => {
// Module: crate::constant
// Provides: {"data_id_for_vtable"}
// Dependencies: {}
pub (crate) fn data_id_for_vtable < 'tcx > (tcx : TyCtxt < 'tcx > , cx : & mut ConstantCx , module : & mut dyn Module , ty : Ty < 'tcx > , trait_ref : Option < ExistentialTraitRef < 'tcx > > ,) -> DataId { let alloc_id = tcx . vtable_allocation ((ty , trait_ref)) ; data_id_for_alloc_id (cx , module , alloc_id , Mutability :: Not) }
};
}
