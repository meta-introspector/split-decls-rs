// Generated macro for buf_ring_reg_and_unreg (function)
macro_rules! Depcrate_tests_register_buf_ringbuf_ring_reg_and_unreg {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"buf_ring_reg_and_unreg"}
// Dependencies: {}
fn buf_ring_reg_and_unreg < S , C > (ring : & mut IoUring < S , C > , _test : & Test) -> io :: Result < () > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { let buf_ring = Builder :: new (777) . ring_entries (16) . buf_len (4096) . build () ? ; buf_ring . rc . register (ring) ? ; buf_ring . rc . unregister (ring) ? ; buf_ring . rc . register (ring) ? ; assert ! (buf_ring . rc . register (ring) . is_err ()) ; buf_ring . rc . unregister (ring) ? ; assert ! (buf_ring . rc . unregister (ring) . is_err ()) ; Ok (()) }
};
}
