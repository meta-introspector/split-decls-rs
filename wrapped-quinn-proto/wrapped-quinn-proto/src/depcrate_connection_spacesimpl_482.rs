// Generated macro for impl_482 (impl)
macro_rules! Depcrate_connection_spacesimpl_482 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_482"}
// Dependencies: {}
impl PacketNumberFilter { pub (super) fn new (rng : & mut (impl Rng + ? Sized)) -> Self { let exponent = 6 ; Self { next_skipped_packet_number : rng . random_range (0 .. 2u64 . saturating_pow (exponent)) , prev_skipped_packet_number : None , exponent , } } # [cfg (test)] pub (super) fn disabled () -> Self { Self { next_skipped_packet_number : u64 :: MAX , prev_skipped_packet_number : None , exponent : u32 :: MAX , } } pub (super) fn peek (& self , space : & PacketSpace) -> u64 { let n = space . next_packet_number ; if n != self . next_skipped_packet_number { return n ; } n + 1 } pub (super) fn allocate (& mut self , rng : & mut (impl Rng + ? Sized) , space : & mut PacketSpace ,) -> u64 { let n = space . get_tx_number () ; if n != self . next_skipped_packet_number { return n ; } trace ! ("skipping pn {n}") ; self . prev_skipped_packet_number = Some (self . next_skipped_packet_number) ; let next_exponent = self . exponent . saturating_add (1) ; self . next_skipped_packet_number = rng . random_range (2u64 . saturating_pow (self . exponent) .. 2u64 . saturating_pow (next_exponent)) ; self . exponent = next_exponent ; space . get_tx_number () } pub (super) fn check_ack (& self , space_id : SpaceId , range : std :: ops :: RangeInclusive < u64 > ,) -> Result < () , TransportError > { if space_id == SpaceId :: Data && self . prev_skipped_packet_number . is_some_and (| x | range . contains (& x)) { return Err (TransportError :: PROTOCOL_VIOLATION ("unsent packet acked")) ; } Ok (()) } }
};
}
