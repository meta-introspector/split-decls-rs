// Generated macro for handle_response_frame (function)
macro_rules! Depcrate_clienthandle_response_frame {
() => {
// Module: crate::client
// Provides: {"handle_response_frame"}
// Dependencies: {}
# [doc = " Push any responses to the [StreamMap] as well as store them in the"] # [doc = " `responded` vector"] fn handle_response_frame < C : Client > (client : & mut C , qlog_streamer : Option < & mut QlogStreamer > , responded_streams : & mut Vec < StreamEvent > , stream_id : u64 , frame : H3iFrame ,) { let cloned = frame . clone () ; client . handle_response_frame (stream_id , cloned) ; let mut to_qlog : Option < Http3Frame > = None ; let mut push_to_responses : Option < StreamEvent > = None ; match frame { H3iFrame :: Headers (enriched_headers) => { push_to_responses = Some (StreamEvent { stream_id , event_type : StreamEventType :: Headers , }) ; let qlog_headers : Vec < HttpHeader > = enriched_headers . headers () . iter () . map (| h | qlog :: events :: h3 :: HttpHeader { name : String :: from_utf8_lossy (h . name ()) . into_owned () , value : String :: from_utf8_lossy (h . value ()) . into_owned () , }) . collect () ; to_qlog = Some (Http3Frame :: Headers { headers : qlog_headers , }) ; } , H3iFrame :: QuicheH3 (quiche_frame) => { if let QFrame :: Data { .. } = quiche_frame { push_to_responses = Some (StreamEvent { stream_id , event_type : StreamEventType :: Data , }) ; } to_qlog = Some (quiche_frame . to_qlog ()) ; } , H3iFrame :: ResetStream (_) => { push_to_responses = Some (StreamEvent { stream_id , event_type : StreamEventType :: Finished , }) ; } , } if let Some (to_qlog) = to_qlog { handle_qlog (qlog_streamer , to_qlog , stream_id) ; } if let Some (to_push) = push_to_responses { responded_streams . push (to_push) ; } }
};
}
