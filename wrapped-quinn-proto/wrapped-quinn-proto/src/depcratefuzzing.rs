// Generated macro for fuzzing (module)
macro_rules! Depcratefuzzing {
() => {
// Module: crate
// Provides: {"fuzzing"}
// Dependencies: {}
# [cfg (fuzzing)] pub mod fuzzing { pub use crate :: connection :: { Retransmits , State as ConnectionState , StreamsState } ; pub use crate :: frame :: ResetStream ; pub use crate :: packet :: PartialDecode ; pub use crate :: transport_parameters :: TransportParameters ; pub use bytes :: { BufMut , BytesMut } ; # [cfg (feature = "arbitrary")] use arbitrary :: { Arbitrary , Result , Unstructured } ; # [cfg (feature = "arbitrary")] impl < 'arbitrary > Arbitrary < 'arbitrary > for TransportParameters { fn arbitrary (u : & mut Unstructured < 'arbitrary >) -> Result < Self > { Ok (Self { initial_max_streams_bidi : u . arbitrary () ? , initial_max_streams_uni : u . arbitrary () ? , ack_delay_exponent : u . arbitrary () ? , max_udp_payload_size : u . arbitrary () ? , .. Self :: default () }) } } # [derive (Debug)] pub struct PacketParams { pub local_cid_len : usize , pub buf : BytesMut , pub grease_quic_bit : bool , } # [cfg (feature = "arbitrary")] impl < 'arbitrary > Arbitrary < 'arbitrary > for PacketParams { fn arbitrary (u : & mut Unstructured < 'arbitrary >) -> Result < Self > { let local_cid_len : usize = u . int_in_range (0 ..= crate :: MAX_CID_SIZE) ? ; let bytes : Vec < u8 > = Vec :: arbitrary (u) ? ; let mut buf = BytesMut :: new () ; buf . put_slice (& bytes [..]) ; Ok (Self { local_cid_len , buf , grease_quic_bit : bool :: arbitrary (u) ? , }) } } }
};
}
