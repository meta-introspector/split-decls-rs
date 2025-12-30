// Generated macro for InFlight (struct)
macro_rules! Depcrate_connection_pathsInFlight {
() => {
// Module: crate::connection::paths
// Provides: {"InFlight"}
// Dependencies: {}
# [doc = " Summary statistics of packets that have been sent on a particular path, but which have not yet"] # [doc = " been acked or deemed lost"] pub (super) struct InFlight { # [doc = " Sum of the sizes of all sent packets considered \"in flight\" by congestion control"] # [doc = ""] # [doc = " The size does not include IP or UDP overhead. Packets only containing ACK frames do not"] # [doc = " count towards this to ensure congestion control does not impede congestion feedback."] pub (super) bytes : u64 , # [doc = " Number of packets in flight containing frames other than ACK and PADDING"] # [doc = ""] # [doc = " This can be 0 even when bytes is not 0 because PADDING frames cause a packet to be"] # [doc = " considered \"in flight\" by congestion control. However, if this is nonzero, bytes will always"] # [doc = " also be nonzero."] pub (super) ack_eliciting : u64 , }
};
}
