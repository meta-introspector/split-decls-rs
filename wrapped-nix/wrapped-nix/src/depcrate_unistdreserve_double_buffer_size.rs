// Generated macro for reserve_double_buffer_size (function)
macro_rules! Depcrate_unistdreserve_double_buffer_size {
() => {
// Module: crate::unistd
// Provides: {"reserve_double_buffer_size"}
// Dependencies: {}
# [cfg (any (feature = "fs" , feature = "user"))] fn reserve_double_buffer_size < T > (buf : & mut Vec < T > , limit : usize) -> Result < () > { use std :: cmp :: min ; if buf . capacity () >= limit { return Err (Errno :: ERANGE) ; } let capacity = min (buf . capacity () * 2 , limit) ; buf . reserve (capacity) ; Ok (()) }
};
}
