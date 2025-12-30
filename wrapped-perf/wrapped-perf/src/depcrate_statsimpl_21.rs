// Generated macro for impl_21 (impl)
macro_rules! Depcrate_statsimpl_21 {
() => {
// Module: crate::stats
// Provides: {"impl_21"}
// Dependencies: {}
impl OpenStreamStats { pub fn new_sender (& self , stream : & quinn :: SendStream , upload_size : u64) -> Arc < StreamStats > { let send_stream_stats = StreamStats { id : stream . id () , request_size : upload_size , bytes : Default :: default () , sender : true , finished : Default :: default () , duration : Default :: default () , first_byte_latency : Default :: default () , } ; let send_stream_stats = Arc :: new (send_stream_stats) ; self . push (send_stream_stats . clone ()) ; send_stream_stats } pub fn new_receiver (& self , stream : & quinn :: RecvStream , download_size : u64) -> Arc < StreamStats > { let recv_stream_stats = StreamStats { id : stream . id () , request_size : download_size , bytes : Default :: default () , sender : false , finished : Default :: default () , duration : Default :: default () , first_byte_latency : Default :: default () , } ; let recv_stream_stats = Arc :: new (recv_stream_stats) ; self . push (recv_stream_stats . clone ()) ; recv_stream_stats } fn push (& self , stream_stats : Arc < StreamStats >) { self . 0 . lock () . unwrap () . push (stream_stats) ; } }
};
}
