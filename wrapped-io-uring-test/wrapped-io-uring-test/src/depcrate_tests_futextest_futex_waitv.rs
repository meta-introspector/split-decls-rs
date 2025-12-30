// Generated macro for test_futex_waitv (function)
macro_rules! Depcrate_tests_futextest_futex_waitv {
() => {
// Module: crate::tests::futex
// Provides: {"test_futex_waitv"}
// Dependencies: {}
pub fn test_futex_waitv < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: FutexWaitV :: CODE) ;) ; const USER_DATA : u64 = 0xDEAD_BEEF_BEEF_DEAD ; println ! ("test futex_waitv") ; const FUTEX_CNT : usize = 5 ; const TRIGGER_IDX : usize = 3 ; let mut futexes = [INIT_VAL ; FUTEX_CNT] ; let mut waitv = [FutexWaitV :: default () ; FUTEX_CNT] ; for (futex , waitv) in futexes . iter () . zip (& mut waitv) { * waitv = FutexWaitV :: new () . val (INIT_VAL as u64) . uaddr (std :: ptr :: from_ref (futex) as _) . flags (FUTEX2_SIZE_U32) ; } let futex_waitv_e = opcode :: FutexWaitV :: new (waitv . as_ptr () . cast () , waitv . len () as _) ; unsafe { let mut queue = ring . submission () ; queue . push (& futex_waitv_e . build () . user_data (USER_DATA) . into ()) . expect ("queue is full") ; } ring . submit () ? ; thread :: sleep (Duration :: from_millis (100)) ; assert_eq ! (ring . completion () . len () , 0) ; futexes [TRIGGER_IDX] += 1 ; let ret = syscall_futex (& futexes [TRIGGER_IDX] , libc :: FUTEX_WAKE , 1) ? ; assert_eq ! (ret , 1) ; ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , USER_DATA) ; assert_eq ! (cqes [0] . result () , TRIGGER_IDX as _) ; Ok (()) }
};
}
