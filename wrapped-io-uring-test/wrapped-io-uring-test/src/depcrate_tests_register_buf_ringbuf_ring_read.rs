// Generated macro for buf_ring_read (function)
macro_rules! Depcrate_tests_register_buf_ringbuf_ring_read {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"buf_ring_read"}
// Dependencies: {}
fn buf_ring_read < S , C > (ring : & mut IoUring < S , C > , buf_ring : & FixedSizeBufRing , fd : types :: Fd , len : u32 ,) -> io :: Result < GBuf > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { let read_e = opcode :: Read :: new (fd , std :: ptr :: null_mut () , len) . offset (0) . buf_group (buf_ring . rc . bgid ()) ; unsafe { let mut queue = ring . submission () ; queue . push (& read_e . build () . user_data (0x02) . flags (squeue :: Flags :: BUFFER_SELECT) . into () ,) . expect ("queue is full") ; } assert_eq ! (ring . submit_and_wait (1) ?, 1) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x02) ; let result = cqes [0] . result () ; if result < 0 { return Err (io :: Error :: from_raw_os_error (- result)) ; } let result = result as u32 ; assert_eq ! (result , len) ; let flags = cqes [0] . flags () ; let buf = buf_ring . rc . get_buf (buf_ring . clone () , result , flags) ? ; Ok (buf) }
};
}
