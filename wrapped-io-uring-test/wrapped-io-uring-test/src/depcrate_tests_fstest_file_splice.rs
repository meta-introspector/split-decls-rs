// Generated macro for test_file_splice (function)
macro_rules! Depcrate_tests_fstest_file_splice {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_splice"}
// Dependencies: {}
pub fn test_file_splice < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { use std :: io :: Read ; require ! (test ; test . probe . is_supported (opcode :: Splice :: CODE) ;) ; println ! ("test file_splice") ; let dir = tempfile :: TempDir :: new_in (".") ? ; let dir = dir . path () ; let input = & [0x9f ; 1024] ; let (pipe_in , mut pipe_out) = { let mut pipes = [0 , 0] ; let ret = unsafe { libc :: pipe (pipes . as_mut_ptr ()) } ; assert_eq ! (ret , 0) ; let pipe_out = unsafe { fs :: File :: from_raw_fd (pipes [0]) } ; let pipe_in = unsafe { fs :: File :: from_raw_fd (pipes [1]) } ; (pipe_in , pipe_out) } ; fs :: write (dir . join ("io-uring-test-file-input") , input) ? ; let fd = fs :: File :: open (dir . join ("io-uring-test-file-input")) ? ; let splice_e = opcode :: Splice :: new (types :: Fd (fd . as_raw_fd ()) , 0 , types :: Fd (pipe_in . as_raw_fd ()) , - 1 , 1024 ,) ; unsafe { ring . submission () . push (& splice_e . build () . user_data (0x33) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x33) ; assert_eq ! (cqes [0] . result () , 1024) ; let mut output = [0 ; 1024] ; pipe_out . read_exact (& mut output) ? ; assert_eq ! (input , & output [..]) ; Ok (()) }
};
}
