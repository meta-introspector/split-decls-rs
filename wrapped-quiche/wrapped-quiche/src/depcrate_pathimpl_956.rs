// Generated macro for impl_956 (impl)
macro_rules! Depcrate_pathimpl_956 {
() => {
// Module: crate::path
// Provides: {"impl_956"}
// Dependencies: {}
impl std :: fmt :: Debug for PathStats { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "local_addr={:?} peer_addr={:?} " , self . local_addr , self . peer_addr ,) ? ; write ! (f , "validation_state={:?} active={} " , self . validation_state , self . active ,) ? ; write ! (f , "recv={} sent={} lost={} retrans={} rtt={:?} min_rtt={:?} rttvar={:?} cwnd={}" , self . recv , self . sent , self . lost , self . retrans , self . rtt , self . min_rtt , self . rttvar , self . cwnd ,) ? ; write ! (f , " sent_bytes={} recv_bytes={} lost_bytes={}" , self . sent_bytes , self . recv_bytes , self . lost_bytes ,) ? ; write ! (f , " stream_retrans_bytes={} pmtu={} delivery_rate={}" , self . stream_retrans_bytes , self . pmtu , self . delivery_rate ,) } }
};
}
