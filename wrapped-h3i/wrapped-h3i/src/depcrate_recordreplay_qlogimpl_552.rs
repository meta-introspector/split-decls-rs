// Generated macro for impl_552 (impl)
macro_rules! Depcrate_recordreplay_qlogimpl_552 {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"impl_552"}
// Dependencies: {}
impl From < & PacketSent > for H3Actions { fn from (ps : & PacketSent) -> Self { let mut actions = vec ! [] ; if let Some (frames) = & ps . frames { for frame in frames { match & frame { QuicFrame :: ResetStream { stream_id , error_code , .. } => actions . push (Action :: ResetStream { stream_id : * stream_id , error_code : * error_code , }) , QuicFrame :: StopSending { stream_id , error_code , .. } => actions . push (Action :: StopSending { stream_id : * stream_id , error_code : * error_code , }) , QuicFrame :: ConnectionClose { error_space , error_code , reason , .. } => { let is_app = matches ! (error_space . as_ref () . expect ("invalid CC frame in qlog input, no error space") , ErrorSpace :: ApplicationError) ; actions . push (Action :: ConnectionClose { error : quiche :: ConnectionError { is_app , error_code : error_code . expect ("invalid CC frame in qlog input, no error code") , reason : reason . as_ref () . map (| s | s . as_bytes () . to_vec ()) . unwrap_or_default () , } , }) } , QuicFrame :: Stream { stream_id , fin , .. } => { let fin = fin . unwrap_or_default () ; if fin { actions . push (Action :: StreamBytes { stream_id : * stream_id , fin_stream : true , bytes : vec ! [] , }) ; } } , QuicFrame :: Datagram { raw , .. } => { actions . push (Action :: SendDatagram { payload : raw . clone () . unwrap_or_default () . into () , }) ; } , _ => () , } } } Self (actions) } }
};
}
