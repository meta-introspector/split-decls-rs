// Generated macro for buf_ring_play (function)
macro_rules! Depcrate_tests_register_buf_ringbuf_ring_play {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"buf_ring_play"}
// Dependencies: {}
fn buf_ring_play < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , _test : & Test ,) -> io :: Result < () > { let text = b"The quick brown fox jumps over the lazy dog." ; let len = text . len () as u32 ; let normal_check = | buf : & GBuf , bid : Bid | { assert_eq ! (bid , buf . bid) ; assert_eq ! (buf . as_slice () , text) ; } ; let buf_ring = Builder :: new (888) . ring_entries (2) . buf_cnt (2) . buf_len (128) . build () ? ; buf_ring . rc . register (ring) ? ; let fd = tempfile :: tempfile () ? ; let fd = types :: Fd (fd . as_raw_fd ()) ; write_text_to_file (ring , fd , text) ? ; let buf0 = buf_ring_read (ring , & buf_ring , fd , len) ? ; let buf1 = buf_ring_read (ring , & buf_ring , fd , len) ? ; normal_check (& buf0 , 0) ; normal_check (& buf1 , 1) ; let res2 = buf_ring_read (ring , & buf_ring , fd , len) ; assert_eq ! (Some (libc :: ENOBUFS) , res2 . unwrap_err () . raw_os_error ()) ; std :: mem :: drop (buf1) ; std :: mem :: drop (buf0) ; let buf3 = buf_ring_read (ring , & buf_ring , fd , len) ? ; let buf4 = buf_ring_read (ring , & buf_ring , fd , len) ? ; normal_check (& buf3 , 1) ; normal_check (& buf4 , 0) ; std :: mem :: drop (buf3) ; std :: mem :: drop (buf4) ; for _ in 0 ..= u16 :: MAX { let _ = buf_ring_read (ring , & buf_ring , fd , len) ? ; } buf_ring . rc . unregister (ring) ? ; Ok (()) }
};
}
