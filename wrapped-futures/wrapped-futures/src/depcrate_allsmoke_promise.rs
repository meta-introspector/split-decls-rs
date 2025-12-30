// Generated macro for smoke_promise (function)
macro_rules! Depcrate_allsmoke_promise {
() => {
// Module: crate::all
// Provides: {"smoke_promise"}
// Dependencies: {}
# [test] fn smoke_promise () { assert_done (| | { let (c , p) = promise () ; c . complete (1) ; p } , Ok (1)) ; assert_done (| | { let (c , p) = promise :: < i32 > () ; drop (c) ; p } , Err (Canceled)) ; let mut completes = Vec :: new () ; assert_empty (| | { let (a , b) = promise :: < i32 > () ; completes . push (a) ; b }) ; let (c , mut p) = promise :: < i32 > () ; drop (c) ; assert ! (p . poll (& mut Task :: new ()) . unwrap () . is_err ()) ; let (c , p) = promise :: < i32 > () ; drop (c) ; let (tx , rx) = channel () ; Task :: new () . run (p . then (move | _ | { tx . send (()) . unwrap () ; Ok (()) }) . boxed ()) ; rx . recv () . unwrap () ; }
};
}
