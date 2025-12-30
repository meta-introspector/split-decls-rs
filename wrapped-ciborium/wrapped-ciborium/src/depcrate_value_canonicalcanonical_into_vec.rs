// Generated macro for canonical_into_vec (function)
macro_rules! Depcrate_value_canonicalcanonical_into_vec {
() => {
// Module: crate::value::canonical
// Provides: {"canonical_into_vec"}
// Dependencies: {}
# [doc = " Serializes an object as CBOR into a new Vec<u8> using RFC 8949 Deterministic Encoding."] # [cfg (feature = "std")] # [inline] pub fn canonical_into_vec < T : ? Sized + ser :: Serialize > (value : & T ,) -> Result < Vec < u8 > , crate :: ser :: Error < < Vec < u8 > as ciborium_io :: Write > :: Error > > { let value = Value :: serialized (value) . map_err (| err | crate :: ser :: Error :: Value (err . to_string ())) ? ; let cvalue = canonical_value (value) ; crate :: into_vec (& cvalue) }
};
}
