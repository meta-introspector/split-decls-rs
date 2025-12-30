// Generated macro for test_futex_wait (function)
macro_rules! Depcrate_tests_futextest_futex_wait {
() => {
// Module: crate::tests::futex
// Provides: {"test_futex_wait"}
// Dependencies: {}
pub fn test_futex_wait < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: FutexWait :: CODE) ;) ; const USER_DATA : u64 = 0xDEAD_BEEF_DEAD_BEEF ; println ! ("test futex_wait") ; let mut futex = INIT_VAL ; let futex_wait_e = opcode :: FutexWait :: new (& futex , INIT_VAL as u64 , libc :: FUTEX_BITSET_MATCH_ANY as u32 as u64 , FUTEX2_SIZE_U32 ,) ; unsafe { let mut queue = ring . submission () ; queue . push (& futex_wait_e . build () . user_data (USER_DATA) . into ()) . expect ("queue is full") ; } ring . submit () ? ; thread :: sleep (Duration :: from_millis (100)) ; assert_eq ! (ring . completion () . len () , 0) ; futex += 1 ; let ret = syscall_futex (& futex , libc :: FUTEX_WAKE , 1) ? ; assert_eq ! (ret , 1) ; ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , USER_DATA) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
