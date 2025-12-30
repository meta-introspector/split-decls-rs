// Generated macro for impl_24 (impl)
macro_rules! Depcrate_internals_attributes_field_boundsimpl_24 {
() => {
// Module: crate::internals::attributes::field::bounds
// Provides: {"impl_24"}
// Dependencies: {}
impl From < BTreeMap < Symbol , Variants > > for Bounds { fn from (mut map : BTreeMap < Symbol , Variants >) -> Self { let serialize = map . remove (& SERIALIZE) ; let deserialize = map . remove (& DESERIALIZE) ; let serialize = serialize . map (| variant | match variant { Variants :: Serialize (ser) => ser , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; let deserialize = deserialize . map (| variant | match variant { Variants :: Deserialize (de) => de , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; Self { serialize , deserialize , } } }
};
}
