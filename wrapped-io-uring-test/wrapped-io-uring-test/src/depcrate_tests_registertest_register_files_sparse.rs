// Generated macro for test_register_files_sparse (function)
macro_rules! Depcrate_tests_registertest_register_files_sparse {
() => {
// Module: crate::tests::register
// Provides: {"test_register_files_sparse"}
// Dependencies: {}
pub fn test_register_files_sparse < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: UringCmd16 :: CODE) ;) ; println ! ("test register_files_sparse") ; ring . submitter () . register_files_sparse (4) ? ; if let Ok (()) = ring . submitter () . register_files_sparse (3) { return Err (anyhow :: anyhow ! ("register_files_sparse should not have succeeded twice in a row")) ; } if let Err (e) = ring . submitter () . unregister_files () { return Err (anyhow :: anyhow ! ("unrgister_files failed: {}" , e)) ; } if let Ok (()) = ring . submitter () . unregister_files () { return Err (anyhow :: anyhow ! ("unrgister_files should not have succeeded twice in a row")) ; } if let Err (e) = ring . submitter () . register_files_sparse (10_000) { if let Some (raw_os_err) = e . raw_os_error () { if raw_os_err == libc :: EMFILE { println ! ("could not open 10,000 file descriptors, try `ulimit -Sn 11000` in the shell") ; return Ok (()) ; } else { return Err (anyhow :: anyhow ! ("register_files_sparse should have succeeded after the previous one was removed: {}" , e)) ; } } else { return Err (anyhow :: anyhow ! ("register_files_sparse should have succeeded after the previous one was removed: {}" , e)) ; } } if let Err (e) = ring . submitter () . unregister_files () { return Err (anyhow :: anyhow ! ("unrgister_files failed, odd since the one could be unregistered earlier: {}" , e)) ; } Ok (()) }
};
}
