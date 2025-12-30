// Generated macro for impl_40 (impl)
macro_rules! Depcrate_internals_attributes_field_schemaimpl_40 {
() => {
// Module: crate::internals::attributes::field::schema
// Provides: {"impl_40"}
// Dependencies: {}
impl From < BTreeMap < Symbol , Variants > > for Attributes { fn from (mut map : BTreeMap < Symbol , Variants >) -> Self { let params = map . remove (& PARAMS) ; let params = params . map (| variant | match variant { Variants :: Params (params) => params , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; let with_funcs = map . remove (& WITH_FUNCS) ; let with_funcs = with_funcs . map (| variant | match variant { Variants :: WithFuncs (with_funcs) => with_funcs , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; Self { params , with_funcs } } }
};
}
