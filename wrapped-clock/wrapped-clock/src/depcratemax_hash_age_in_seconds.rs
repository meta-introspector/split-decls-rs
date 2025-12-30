// Generated macro for MAX_HASH_AGE_IN_SECONDS (const)
macro_rules! DepcrateMAX_HASH_AGE_IN_SECONDS {
() => {
// Module: crate
// Provides: {"MAX_HASH_AGE_IN_SECONDS"}
// Dependencies: {}
# [doc = " The time window of recent block hash values over which the bank will track"] # [doc = " signatures."] # [doc = ""] # [doc = " Once the bank discards a block hash, it will reject any transactions that"] # [doc = " use that `recent_blockhash` in a transaction. Lowering this value reduces"] # [doc = " memory consumption, but requires a client to update its `recent_blockhash`"] # [doc = " more frequently. Raising the value lengthens the time a client must wait to"] # [doc = " be certain a missing transaction will not be processed by the network."] pub const MAX_HASH_AGE_IN_SECONDS : usize = 120 ;
};
}
