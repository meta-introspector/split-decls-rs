// Generated macro for canonical_into_writer (function)
macro_rules! Depcrate_value_canonicalcanonical_into_writer {
() => {
// Module: crate::value::canonical
// Provides: {"canonical_into_writer"}
// Dependencies: {}
# [doc = " Serializes an object as CBOR into a writer using RFC 8949 Deterministic Encoding."] # [inline] pub fn canonical_into_writer < T : ? Sized + ser :: Serialize , W : Write > (value : & T , writer : W ,) -> Result < () , crate :: ser :: Error < W :: Error > > where W :: Error : core :: fmt :: Debug , { let value = Value :: serialized (value) . map_err (| err | crate :: ser :: Error :: Value (err . to_string ())) ? ; let cvalue = canonical_value (value) ; crate :: into_writer (& cvalue , writer) }
};
}
