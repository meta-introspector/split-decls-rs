// Generated macro for use_309 (use)
macro_rules! Depcrate_connectionuse_309 {
() => {
// Module: crate::connection
// Provides: {"use_309"}
// Dependencies: {}
use crate :: { Dir , Duration , EndpointConfig , Frame , INITIAL_MTU , Instant , MAX_CID_SIZE , MAX_STREAM_COUNT , MIN_INITIAL_SIZE , Side , StreamId , TIMER_GRANULARITY , TokenStore , Transmit , TransportError , TransportErrorCode , VarInt , cid_generator :: ConnectionIdGenerator , cid_queue :: CidQueue , coding :: BufMutExt , config :: { ServerConfig , TransportConfig } , connection :: spaces :: LostPacket , crypto :: { self , KeyPair , Keys , PacketKey } , frame :: { self , Close , Datagram , FrameStruct , NewConnectionId , NewToken } , packet :: { FixedLengthConnectionIdParser , Header , InitialHeader , InitialPacket , LongType , Packet , PacketNumber , PartialDecode , SpaceId , } , range_set :: ArrayRangeSet , shared :: { ConnectionEvent , ConnectionEventInner , ConnectionId , DatagramConnectionEvent , EcnCodepoint , EndpointEvent , EndpointEventInner , } , token :: { ResetToken , Token , TokenPayload } , transport_parameters :: TransportParameters , } ;
};
}
