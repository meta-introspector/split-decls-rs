// Generated macro for test_futex_wake (function)
macro_rules! Depcrate_tests_futextest_futex_wake {
() => {
// Module: crate::tests::futex
// Provides: {"test_futex_wake"}
// Dependencies: {}
pub fn test_futex_wake < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: FutexWake :: CODE) ;) ; const USER_DATA : u64 = 0xBEEF_DEAD_BEEF_DEAD ; println ! ("test futex_wake") ; let futex = Arc :: new (AtomicU32 :: new (INIT_VAL)) ; let wait_thread = std :: thread :: spawn ({ let futex = futex . clone () ; move | | { while futex . load (Ordering :: Relaxed) == INIT_VAL { let ret = syscall_futex (futex . as_ptr () , libc :: FUTEX_WAIT , INIT_VAL) ; assert_eq ! (ret . unwrap () , 0) ; } } }) ; thread :: sleep (Duration :: from_millis (100)) ; assert ! (! wait_thread . is_finished ()) ; futex . store (INIT_VAL + 1 , Ordering :: Relaxed) ; let futex_wake_e = opcode :: FutexWake :: new (futex . as_ptr () , 1 , libc :: FUTEX_BITSET_MATCH_ANY as u32 as u64 , FUTEX2_SIZE_U32 ,) ; unsafe { let mut queue = ring . submission () ; queue . push (& futex_wake_e . build () . user_data (USER_DATA) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , USER_DATA) ; assert_eq ! (cqes [0] . result () , 1) ; wait_thread . join () . unwrap () ; Ok (()) }
};
}
