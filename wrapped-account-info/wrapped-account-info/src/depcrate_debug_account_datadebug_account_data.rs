// Generated macro for debug_account_data (function)
macro_rules! Depcrate_debug_account_datadebug_account_data {
() => {
// Module: crate::debug_account_data
// Provides: {"debug_account_data"}
// Dependencies: {}
# [doc = " Format data as hex."] # [doc = ""] # [doc = " If `data`'s length is greater than 0, add a field called \"data\" to `f`. The"] # [doc = " first 64 bytes of `data` is displayed; bytes after that are ignored."] pub fn debug_account_data (data : & [u8] , f : & mut fmt :: DebugStruct < '_ , '_ >) { let data_len = cmp :: min (MAX_DEBUG_ACCOUNT_DATA , data . len ()) ; if data_len > 0 { f . field ("data" , & Hex (& data [.. data_len])) ; } }
};
}
