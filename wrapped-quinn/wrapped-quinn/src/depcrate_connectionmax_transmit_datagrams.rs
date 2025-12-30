// Generated macro for MAX_TRANSMIT_DATAGRAMS (const)
macro_rules! Depcrate_connectionMAX_TRANSMIT_DATAGRAMS {
() => {
// Module: crate::connection
// Provides: {"MAX_TRANSMIT_DATAGRAMS"}
// Dependencies: {}
# [doc = " The maximum amount of datagrams which will be produced in a single `drive_transmit` call"] # [doc = ""] # [doc = " This limits the amount of CPU resources consumed by datagram generation,"] # [doc = " and allows other tasks (like receiving ACKs) to run in between."] const MAX_TRANSMIT_DATAGRAMS : usize = 20 ;
};
}
