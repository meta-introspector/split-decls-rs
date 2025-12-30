// Generated macro for impl_23 (impl)
macro_rules! Depcrate_statsimpl_23 {
() => {
// Module: crate::stats
// Provides: {"impl_23"}
// Dependencies: {}
impl StreamStats { pub fn on_first_byte (& self , latency : Duration) { self . first_byte_latency . store (latency . as_micros () as u64 , Ordering :: SeqCst) ; } pub fn on_bytes (& self , bytes : usize) { self . bytes . fetch_add (bytes , Ordering :: SeqCst) ; } pub fn finish (& self , duration : Duration) { self . duration . store (duration . as_micros () as u64 , Ordering :: SeqCst) ; self . finished . store (true , Ordering :: SeqCst) ; } }
};
}
