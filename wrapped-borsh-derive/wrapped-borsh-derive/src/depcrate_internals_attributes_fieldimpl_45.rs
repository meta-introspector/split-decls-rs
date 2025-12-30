// Generated macro for impl_45 (impl)
macro_rules! Depcrate_internals_attributes_fieldimpl_45 {
() => {
// Module: crate::internals::attributes::field
// Provides: {"impl_45"}
// Dependencies: {}
impl From < BTreeMap < Symbol , Variants > > for Attributes { fn from (mut map : BTreeMap < Symbol , Variants >) -> Self { let bounds = map . remove (& BOUND) ; let serialize_with = map . remove (& SERIALIZE_WITH) ; let deserialize_with = map . remove (& DESERIALIZE_WITH) ; let skip = map . remove (& SKIP) ; let bounds = bounds . map (| variant | match variant { Variants :: Bounds (bounds) => bounds , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; let serialize_with = serialize_with . map (| variant | match variant { Variants :: SerializeWith (serialize_with) => serialize_with , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; let deserialize_with = deserialize_with . map (| variant | match variant { Variants :: DeserializeWith (deserialize_with) => deserialize_with , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; let skip = skip . map (| variant | match variant { Variants :: Skip (skip) => skip , _ => unreachable ! ("only one enum variant is expected to correspond to given map key") , }) ; # [cfg (feature = "schema")] let schema = { let schema = map . remove (& SCHEMA) ; schema . map (| variant | match variant { Variants :: Schema (schema) => schema , _ => { unreachable ! ("only one enum variant is expected to correspond to given map key") } }) } ; Self { bounds , serialize_with , deserialize_with , skip : skip . is_some () , # [cfg (feature = "schema")] schema , } } }
};
}
