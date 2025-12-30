// Generated macro for decode_oid_stat (function)
macro_rules! Depcrate_extension_untracked_cachedecode_oid_stat {
() => {
// Module: crate::extension::untracked_cache
// Provides: {"decode_oid_stat"}
// Dependencies: {}
fn decode_oid_stat (data : & [u8] , hash_len : usize) -> Option < (OidStat , & [u8]) > { let (stat , data) = crate :: decode :: stat (data) ? ; let (hash , data) = data . split_at_checked (hash_len) ? ; Some ((OidStat { stat , id : ObjectId :: from_bytes_or_panic (hash) , } , data ,)) }
};
}
