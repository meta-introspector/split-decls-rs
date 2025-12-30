// Generated macro for test_sqpoll (function)
macro_rules! Depcratetest_sqpoll {
() => {
// Module: crate
// Provides: {"test_sqpoll"}
// Dependencies: {}
fn test_sqpoll < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (mut ring : IoUring < S , C > ,) -> anyhow :: Result < () > { if ! ring . params () . is_setup_sqpoll () { return Err (anyhow :: anyhow ! ("IORING_SETUP_SQPOLL must be enabled to run this test")) ; } let mut probe = Probe :: new () ; if ring . submitter () . register_probe (& mut probe) . is_err () { eprintln ! ("No probe supported") ; } println ! () ; println ! ("ring type: IoUring<{}, {}>" , std :: any :: type_name ::< S > () . strip_prefix ("io_uring::") . unwrap () , std :: any :: type_name ::< C > () . strip_prefix ("io_uring::") . unwrap () ,) ; println ! ("params: {:#?}" , ring . params ()) ; println ! ("probe: {:?}" , probe) ; println ! () ; let test = Test { probe , target : std :: env :: args () . nth (1) , count : Cell :: new (0) , } ; tests :: sqpoll :: test_sqpoll_cq_overflow (& mut ring , & test) ? ; println ! ("Test count: {}" , test . count . get ()) ; Ok (()) }
};
}
