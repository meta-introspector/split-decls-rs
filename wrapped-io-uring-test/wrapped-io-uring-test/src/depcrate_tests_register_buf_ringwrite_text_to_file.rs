// Generated macro for write_text_to_file (function)
macro_rules! Depcrate_tests_register_buf_ringwrite_text_to_file {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"write_text_to_file"}
// Dependencies: {}
fn write_text_to_file < S , C > (ring : & mut IoUring < S , C > , fd : types :: Fd , text : & [u8]) -> io :: Result < () > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { let write_e = opcode :: Write :: new (fd , text . as_ptr () , text . len () as _) ; unsafe { let mut queue = ring . submission () ; let write_e = write_e . build () . user_data (0x01) . flags (squeue :: Flags :: IO_LINK) . into () ; queue . push (& write_e) . expect ("queue is full") ; } assert_eq ! (ring . submit_and_wait (1) ?, 1) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x01) ; assert_eq ! (cqes [0] . result () , text . len () as i32) ; Ok (()) }
};
}
