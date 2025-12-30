// Generated macro for join_cancels (function)
macro_rules! Depcrate_alljoin_cancels {
() => {
// Module: crate::all
// Provides: {"join_cancels"}
// Dependencies: {}
# [test] fn join_cancels () { let ((a , b) , (c , d)) = (promise :: < i32 > () , promise :: < i32 > ()) ; let ((btx , _brx) , (dtx , drx)) = (channel () , channel ()) ; let b = b . map (move | b | { btx . send (b) . unwrap () ; b }) ; let d = d . map (move | d | { dtx . send (d) . unwrap () ; d }) ; let mut f = b . join (d) ; drop (a) ; assert ! (f . poll (& mut Task :: new ()) . is_ready ()) ; drop ((c , f)) ; assert ! (drx . recv () . is_err ()) ; let ((a , b) , (c , d)) = (promise :: < i32 > () , promise :: < i32 > ()) ; let ((btx , _brx) , (dtx , drx)) = (channel () , channel ()) ; let b = b . map (move | b | { btx . send (b) . unwrap () ; b }) ; let d = d . map (move | d | { dtx . send (d) . unwrap () ; d }) ; let (tx , rx) = channel () ; let f = b . join (d) ; Task :: new () . run (f . then (move | _ | { tx . send (()) . unwrap () ; Ok (()) }) . boxed ()) ; assert ! (rx . try_recv () . is_err ()) ; drop (a) ; rx . recv () . unwrap () ; drop (c) ; assert ! (drx . recv () . is_err ()) ; }
};
}
