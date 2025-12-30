// Generated macro for stream_ids (function)
macro_rules! Depcrate_wirefilterstream_ids {
() => {
// Module: crate::wirefilter
// Provides: {"stream_ids"}
// Dependencies: {}
fn stream_ids (event : & Event) -> TypedArray < '_ , i64 > { let mut ids : TypedArray < i64 > = TypedArray :: new () ; match event { Event :: Qlog (event) => match & event . data { EventData :: DataMoved (v) => if let Some (id) = v . stream_id { ids . push (id as i64) ; } , EventData :: PacketSent (v) => { if let Some (frames) = & v . frames { for frame in frames { match frame { qlog :: events :: quic :: QuicFrame :: ResetStream { stream_id , .. } => ids . push (* stream_id as i64) , qlog :: events :: quic :: QuicFrame :: StopSending { stream_id , .. } => ids . push (* stream_id as i64) , qlog :: events :: quic :: QuicFrame :: Stream { stream_id , .. } => ids . push (* stream_id as i64) , qlog :: events :: quic :: QuicFrame :: MaxStreamData { stream_id , .. } => ids . push (* stream_id as i64) , qlog :: events :: quic :: QuicFrame :: StreamDataBlocked { stream_id , .. } => ids . push (* stream_id as i64) , _ => () , } } } } , EventData :: H3StreamTypeSet (v) => { ids . push (v . stream_id as i64) ; } , EventData :: H3FrameCreated (v) => { ids . push (v . stream_id as i64) ; } , EventData :: H3FrameParsed (v) => { ids . push (v . stream_id as i64) ; } , _ => () , } , Event :: Json (_event) => { } , } ids }
};
}
