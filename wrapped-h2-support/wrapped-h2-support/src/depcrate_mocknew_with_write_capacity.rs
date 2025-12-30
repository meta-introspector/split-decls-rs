// Generated macro for new_with_write_capacity (function)
macro_rules! Depcrate_mocknew_with_write_capacity {
() => {
// Module: crate::mock
// Provides: {"new_with_write_capacity"}
// Dependencies: {}
# [doc = " Create a new mock and handle allowing up to `cap` bytes to be written."] pub fn new_with_write_capacity (cap : usize) -> (Mock , Handle) { let inner = Arc :: new (Mutex :: new (Inner { rx : vec ! [] , rx_task : None , tx : vec ! [] , tx_task : None , tx_rem : cap , tx_rem_task : None , closed : false , unexpected_eof : false , })) ; let mock = Mock { pipe : Pipe { inner : inner . clone () , } , } ; let handle = Handle { codec : h2 :: Codec :: new (Pipe { inner }) , } ; (mock , handle) }
};
}
