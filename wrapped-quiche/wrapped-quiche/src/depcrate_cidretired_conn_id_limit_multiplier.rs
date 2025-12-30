// Generated macro for RETIRED_CONN_ID_LIMIT_MULTIPLIER (const)
macro_rules! Depcrate_cidRETIRED_CONN_ID_LIMIT_MULTIPLIER {
() => {
// Module: crate::cid
// Provides: {"RETIRED_CONN_ID_LIMIT_MULTIPLIER"}
// Dependencies: {}
# [doc = " Used to calculate the cap for the queue of retired connection IDs for which"] # [doc = " a RETIRED_CONNECTION_ID frame have not been sent, as a multiple of"] # [doc = " `active_conn_id_limit` (see RFC 9000, section 5.1.2)."] const RETIRED_CONN_ID_LIMIT_MULTIPLIER : u64 = 3 ;
};
}
