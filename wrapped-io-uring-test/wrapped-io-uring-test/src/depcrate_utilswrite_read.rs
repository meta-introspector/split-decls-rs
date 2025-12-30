// Generated macro for write_read (function)
macro_rules! Depcrate_utilswrite_read {
() => {
// Module: crate::utils
// Provides: {"write_read"}
// Dependencies: {}
pub fn write_read < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , fd_in : types :: Fd , fd_out : types :: Fd ,) -> anyhow :: Result < () > { let text = b"The quick brown fox jumps over the lazy dog." ; let mut output = vec ! [0 ; text . len ()] ; let write_e = opcode :: Write :: new (fd_in , text . as_ptr () , text . len () as _) ; let read_e = opcode :: Read :: new (fd_out , output . as_mut_ptr () , output . len () as _) ; unsafe { let mut queue = ring . submission () ; let write_e = write_e . build () . user_data (0x01) . flags (squeue :: Flags :: IO_LINK) . into () ; queue . push (& write_e) . expect ("queue is full") ; queue . push (& read_e . build () . user_data (0x02) . into ()) . expect ("queue is full") ; } assert_eq ! (ring . submit_and_wait (2) ?, 2) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x01) ; assert_eq ! (cqes [1] . user_data () , 0x02) ; assert_eq ! (cqes [0] . result () , text . len () as i32) ; assert_eq ! (cqes [1] . result () , text . len () as i32) ; assert_eq ! (& output [.. cqes [1] . result () as usize] , text) ; Ok (()) }
};
}
