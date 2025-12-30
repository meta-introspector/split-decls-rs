// Generated macro for test_file_openat2 (function)
macro_rules! Depcrate_tests_fstest_file_openat2 {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_openat2"}
// Dependencies: {}
pub fn test_file_openat2 < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: OpenAt2 :: CODE) ;) ; use tempfile :: tempdir ; println ! ("test file_openat2") ; let dir = tempdir () ? ; let dirfd = types :: Fd (libc :: AT_FDCWD) ; let path = dir . path () . join ("test-io-uring-openat2") ; let path = CString :: new (path . as_os_str () . as_bytes ()) ? ; let openhow = types :: OpenHow :: new () . flags (libc :: O_CREAT as _) ; let open_e = opcode :: OpenAt2 :: new (dirfd , path . as_ptr () , & openhow) ; unsafe { ring . submission () . push (& open_e . build () . user_data (0x11) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x11) ; assert ! (cqes [0] . result () > 0) ; let fd = unsafe { fs :: File :: from_raw_fd (cqes [0] . result ()) } ; assert ! (fd . metadata () ?. is_file ()) ; Ok (()) }
};
}
