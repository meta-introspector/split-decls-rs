// Generated macro for PacketNumberFilter (struct)
macro_rules! Depcrate_connection_spacesPacketNumberFilter {
() => {
// Module: crate::connection::spaces
// Provides: {"PacketNumberFilter"}
// Dependencies: {}
# [doc = " Helper for mitigating [optimistic ACK attacks]"] # [doc = ""] # [doc = " A malicious peer could prompt the local application to begin a large data transfer, and then"] # [doc = " send ACKs without first waiting for data to be received. This could defeat congestion control,"] # [doc = " allowing the connection to consume disproportionate resources. We therefore occasionally skip"] # [doc = " packet numbers, and classify any ACK referencing a skipped packet number as a transport error."] # [doc = ""] # [doc = " Skipped packet numbers occur only in the application data space (where costly transfers might"] # [doc = " take place) and are distributed exponentially to reflect the reduced likelihood and impact of"] # [doc = " bad behavior from a peer that has been well-behaved for an extended period."] # [doc = ""] # [doc = " ACKs for packet numbers that have not yet been allocated are also a transport error, but an"] # [doc = " attacker with knowledge of the congestion control algorithm in use could time falsified ACKs to"] # [doc = " arrive after the packets they reference are sent."] # [doc = ""] # [doc = " [optimistic ACK attacks]: https://www.rfc-editor.org/rfc/rfc9000.html#name-optimistic-ack-attack"] pub (super) struct PacketNumberFilter { # [doc = " Next outgoing packet number to skip"] next_skipped_packet_number : u64 , # [doc = " Most recently skipped packet number"] prev_skipped_packet_number : Option < u64 > , # [doc = " Next packet number to skip is randomly selected from 2^n..2^n+1"] exponent : u32 , }
};
}
