// Generated macro for test_register_sync_cancel_any (function)
macro_rules! Depcrate_tests_register_sync_canceltest_register_sync_cancel_any {
() => {
// Module: crate::tests::register_sync_cancel
// Provides: {"test_register_sync_cancel_any"}
// Dependencies: {}
pub fn test_register_sync_cancel_any < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> io :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: SendZc :: CODE) ;) ; let fd_1 = get_eventfd () ; const START_USER_DATA : u64 = 47u64 ; let mut buf = [0u8 ; 32] ; for i in 0 .. 3 { let entry = opcode :: Read :: new (types :: Fd (fd_1 . as_raw_fd ()) , buf . as_mut_ptr () , 32) . build () . user_data (START_USER_DATA + i) ; unsafe { ring . submission () . push (& entry . into ()) . unwrap () } ; } assert_eq ! (3 , ring . submit () ?) ; ring . submitter () . register_sync_cancel (None , CancelBuilder :: any () . all ()) ? ; let completions = wait_get_completions (ring , 5) . unwrap () ; assert_eq ! (completions . len () , 3) ; for completion in completions . iter () { assert_eq ! (completion . result () , - libc :: ECANCELED) ; } let mut user_data_entries = completions . into_iter () . map (| c | c . user_data ()) . collect :: < Vec < u64 > > () ; user_data_entries . sort () ; assert_eq ! (user_data_entries , vec ! [START_USER_DATA , START_USER_DATA + 1 , START_USER_DATA + 2 ,]) ; Ok (()) }
};
}
