// Generated macro for data_range (function)
macro_rules! Depcrate_read_utildata_range {
() => {
// Module: crate::read::util
// Provides: {"data_range"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn data_range (data : & [u8] , data_address : u64 , range_address : u64 , size : u64 ,) -> Option < & [u8] > { let offset = range_address . checked_sub (data_address) ? ; data . get (offset . try_into () . ok () ? ..) ? . get (.. size . try_into () . ok () ?) }
};
}
