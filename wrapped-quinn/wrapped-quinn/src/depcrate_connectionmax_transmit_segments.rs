// Generated macro for MAX_TRANSMIT_SEGMENTS (const)
macro_rules! Depcrate_connectionMAX_TRANSMIT_SEGMENTS {
() => {
// Module: crate::connection
// Provides: {"MAX_TRANSMIT_SEGMENTS"}
// Dependencies: {}
# [doc = " The maximum amount of datagrams that are sent in a single transmit"] # [doc = ""] # [doc = " This can be lower than the maximum platform capabilities, to avoid excessive"] # [doc = " memory allocations when calling `poll_transmit()`. Benchmarks have shown"] # [doc = " that numbers around 10 are a good compromise."] const MAX_TRANSMIT_SEGMENTS : usize = 10 ;
};
}
