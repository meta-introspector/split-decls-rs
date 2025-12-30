// Generated macro for try_to_vec_with_schema (function)
macro_rules! Depcrate_schema_helperstry_to_vec_with_schema {
() => {
// Module: crate::schema_helpers
// Provides: {"try_to_vec_with_schema"}
// Dependencies: {}
# [doc = " Serialize object into a vector of bytes and prefix with the schema serialized as vector of"] # [doc = " bytes in Borsh format."] pub fn try_to_vec_with_schema < T : BorshSerialize + BorshSchema + ? Sized > (value : & T ,) -> Result < Vec < u8 > > { let schema = schema_container_of :: < T > () ; let mut res = crate :: to_vec (& schema) ? ; value . serialize (& mut res) ? ; Ok (res) }
};
}
