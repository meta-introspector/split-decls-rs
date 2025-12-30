// Generated macro for try_from_slice_with_schema (function)
macro_rules! Depcrate_schema_helperstry_from_slice_with_schema {
() => {
// Module: crate::schema_helpers
// Provides: {"try_from_slice_with_schema"}
// Dependencies: {}
# [doc = " Deserialize this instance from a slice of bytes, but assume that at the beginning we have"] # [doc = " bytes describing the schema of the type. We deserialize this schema and verify that it is"] # [doc = " correct."] pub fn try_from_slice_with_schema < T : BorshDeserialize + BorshSchema > (v : & [u8]) -> Result < T > { let (schema , object) = from_slice :: < (BorshSchemaContainer , T) > (v) ? ; if schema_container_of :: < T > () != schema { return Err (Error :: new (ErrorKind :: InvalidData , "Borsh schema does not match" ,)) ; } Ok (object) }
};
}
