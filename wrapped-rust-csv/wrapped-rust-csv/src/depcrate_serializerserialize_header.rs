// Generated macro for serialize_header (function)
macro_rules! Depcrate_serializerserialize_header {
() => {
// Module: crate::serializer
// Provides: {"serialize_header"}
// Dependencies: {}
# [doc = " Write header names corresponding to the field names of the value (if the"] # [doc = " value has field names)."] # [doc = ""] # [doc = " If the type to be serialized has field names (e.g. it's a struct), then"] # [doc = " header names are written, and the `Ok` return value is `true`."] # [doc = ""] # [doc = " If the type to be serialized doesn't have field names, then nothing is"] # [doc = " written, and the `Ok` return value is `false`."] pub fn serialize_header < S : Serialize , W : io :: Write > (wtr : & mut Writer < W > , value : S ,) -> Result < bool , Error > { let mut ser = SeHeader :: new (wtr) ; value . serialize (& mut ser) . map (| _ | ser . wrote_header ()) }
};
}
