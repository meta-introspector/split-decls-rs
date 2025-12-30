// Generated macro for select_cancels (function)
macro_rules! Depcrate_allselect_cancels {
() => {
// Module: crate::all
// Provides: {"select_cancels"}
// Dependencies: {}
# [test] fn select_cancels () { let ((a , b) , (c , d)) = (promise :: < i32 > () , promise :: < i32 > ()) ; let ((btx , brx) , (dtx , drx)) = (channel () , channel ()) ; let b = b . map (move | b | { btx . send (b) . unwrap () ; b }) ; let d = d . map (move | d | { dtx . send (d) . unwrap () ; d }) ; let mut f = b . select (d) . then (unselect) ; assert ! (brx . try_recv () . is_err ()) ; assert ! (drx . try_recv () . is_err ()) ; a . complete (1) ; assert ! (f . poll (& mut Task :: new ()) . is_ready ()) ; assert_eq ! (brx . recv () . unwrap () , 1) ; drop ((c , f)) ; assert ! (drx . recv () . is_err ()) ; let ((a , b) , (c , d)) = (promise :: < i32 > () , promise :: < i32 > ()) ; let ((btx , _brx) , (dtx , drx)) = (channel () , channel ()) ; let b = b . map (move | b | { btx . send (b) . unwrap () ; b }) ; let d = d . map (move | d | { dtx . send (d) . unwrap () ; d }) ; let mut f = b . select (d) . then (unselect) ; let mut task = Task :: new () ; assert ! (f . poll (& mut task) . is_not_ready ()) ; f . schedule (& mut task) ; assert ! (f . poll (& mut task) . is_not_ready ()) ; a . complete (1) ; assert ! (f . poll (& mut task) . is_ready ()) ; drop ((c , f)) ; assert ! (drx . recv () . is_err ()) ; }
};
}
