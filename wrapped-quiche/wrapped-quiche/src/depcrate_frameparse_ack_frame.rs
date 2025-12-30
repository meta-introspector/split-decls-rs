// Generated macro for parse_ack_frame (function)
macro_rules! Depcrate_frameparse_ack_frame {
() => {
// Module: crate::frame
// Provides: {"parse_ack_frame"}
// Dependencies: {}
fn parse_ack_frame (ty : u64 , b : & mut octets :: Octets) -> Result < Frame > { let first = ty as u8 ; let largest_ack = b . get_varint () ? ; let ack_delay = b . get_varint () ? ; let block_count = b . get_varint () ? ; let ack_block = b . get_varint () ? ; if largest_ack < ack_block { return Err (Error :: InvalidFrame) ; } let mut smallest_ack = largest_ack - ack_block ; let mut ranges = ranges :: RangeSet :: default () ; ranges . insert (smallest_ack .. largest_ack + 1) ; for _i in 0 .. block_count { let gap = b . get_varint () ? ; if smallest_ack < 2 + gap { return Err (Error :: InvalidFrame) ; } let largest_ack = (smallest_ack - gap) - 2 ; let ack_block = b . get_varint () ? ; if largest_ack < ack_block { return Err (Error :: InvalidFrame) ; } smallest_ack = largest_ack - ack_block ; ranges . insert (smallest_ack .. largest_ack + 1) ; } let ecn_counts = if first & 0x01 != 0 { let ecn = EcnCounts { ect0_count : b . get_varint () ? , ect1_count : b . get_varint () ? , ecn_ce_count : b . get_varint () ? , } ; Some (ecn) } else { None } ; Ok (Frame :: ACK { ack_delay , ranges , ecn_counts , }) }
};
}
