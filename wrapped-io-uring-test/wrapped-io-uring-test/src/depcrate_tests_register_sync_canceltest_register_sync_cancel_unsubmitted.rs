// Generated macro for test_register_sync_cancel_unsubmitted (function)
macro_rules! Depcrate_tests_register_sync_canceltest_register_sync_cancel_unsubmitted {
() => {
// Module: crate::tests::register_sync_cancel
// Provides: {"test_register_sync_cancel_unsubmitted"}
// Dependencies: {}
pub fn test_register_sync_cancel_unsubmitted < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> io :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: SendZc :: CODE) ;) ; let fd_1 = get_eventfd () ; const USER_DATA : u64 = 47u64 ; let mut buf = [0u8 ; 32] ; let entry = opcode :: Read :: new (types :: Fd (fd_1 . as_raw_fd ()) , buf . as_mut_ptr () , 32) . build () . user_data (USER_DATA) ; unsafe { ring . submission () . push (& entry . into ()) . unwrap () } ; let result = ring . submitter () . register_sync_cancel (None , CancelBuilder :: user_data (USER_DATA)) ; assert ! (matches ! (result . err () . unwrap () . kind () , io :: ErrorKind :: NotFound) , "the operation should not complete because the entry has not been submitted") ; assert_eq ! (1 , ring . submitter () . submit () ?) ; ring . submitter () . register_sync_cancel (None , CancelBuilder :: user_data (USER_DATA)) ? ; let completions = wait_get_completions (ring , 1) ? ; assert_eq ! (completions . len () , 1) ; assert_eq ! (completions [0] . user_data () , USER_DATA) ; assert_eq ! (completions [0] . result () , - libc :: ECANCELED) ; Ok (()) }
};
}
