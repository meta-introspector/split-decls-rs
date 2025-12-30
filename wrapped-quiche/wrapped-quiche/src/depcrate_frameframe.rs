// Generated macro for Frame (enum)
macro_rules! Depcrate_frameFrame {
() => {
// Module: crate::frame
// Provides: {"Frame"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq)] pub enum Frame { Padding { len : usize , } , Ping { mtu_probe : Option < usize > , } , ACK { ack_delay : u64 , ranges : ranges :: RangeSet , ecn_counts : Option < EcnCounts > , } , ResetStream { stream_id : u64 , error_code : u64 , final_size : u64 , } , StopSending { stream_id : u64 , error_code : u64 , } , Crypto { data : RangeBuf , } , CryptoHeader { offset : u64 , length : usize , } , NewToken { token : Vec < u8 > , } , Stream { stream_id : u64 , data : RangeBuf , } , StreamHeader { stream_id : u64 , offset : u64 , length : usize , fin : bool , } , MaxData { max : u64 , } , MaxStreamData { stream_id : u64 , max : u64 , } , MaxStreamsBidi { max : u64 , } , MaxStreamsUni { max : u64 , } , DataBlocked { limit : u64 , } , StreamDataBlocked { stream_id : u64 , limit : u64 , } , StreamsBlockedBidi { limit : u64 , } , StreamsBlockedUni { limit : u64 , } , NewConnectionId { seq_num : u64 , retire_prior_to : u64 , conn_id : Vec < u8 > , reset_token : [u8 ; 16] , } , RetireConnectionId { seq_num : u64 , } , PathChallenge { data : [u8 ; 8] , } , PathResponse { data : [u8 ; 8] , } , ConnectionClose { error_code : u64 , frame_type : u64 , reason : Vec < u8 > , } , ApplicationClose { error_code : u64 , reason : Vec < u8 > , } , HandshakeDone , Datagram { data : Vec < u8 > , } , DatagramHeader { length : usize , } , }
};
}
