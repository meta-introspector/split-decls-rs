// Generated macro for writev_readv (function)
macro_rules! Depcrate_utilswritev_readv {
() => {
// Module: crate::utils
// Provides: {"writev_readv"}
// Dependencies: {}
pub fn writev_readv < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , fd_in : types :: Fd , fd_out : types :: Fd ,) -> anyhow :: Result < () > { let text = b"The quick brown fox jumps over the lazy dog." ; let text2 = "我能吞下玻璃而不伤身体。" . as_bytes () ; let mut output = vec ! [0 ; text . len ()] ; let mut output2 = vec ! [0 ; text2 . len ()] ; let text3 = [IoSlice :: new (text) , IoSlice :: new (text2)] ; let mut output3 = vec ! [IoSliceMut :: new (& mut output) , IoSliceMut :: new (& mut output2)] ; let write_e = opcode :: Writev :: new (fd_in , text3 . as_ptr () . cast () , text3 . len () as _) ; let read_e = opcode :: Readv :: new (fd_out , output3 . as_mut_ptr () . cast () , output3 . len () as _) ; unsafe { let mut queue = ring . submission () ; let write_e = write_e . build () . user_data (0x01) . flags (squeue :: Flags :: IO_LINK) . into () ; queue . push (& write_e) . expect ("queue is full") ; queue . push (& read_e . build () . user_data (0x02) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (2) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x01) ; assert_eq ! (cqes [1] . user_data () , 0x02) ; assert_eq ! (cqes [0] . result () , (text . len () + text2 . len ()) as i32) ; assert_eq ! (cqes [1] . result () , (text . len () + text2 . len ()) as i32) ; assert_eq ! (& output , text) ; assert_eq ! (& output2 [..] , text2) ; Ok (()) }
};
}
