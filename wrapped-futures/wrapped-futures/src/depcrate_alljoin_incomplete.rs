// Generated macro for join_incomplete (function)
macro_rules! Depcrate_alljoin_incomplete {
() => {
// Module: crate::all
// Provides: {"join_incomplete"}
// Dependencies: {}
# [test] fn join_incomplete () { let (a , b) = promise :: < i32 > () ; let mut f = finished (1) . join (b) ; assert ! (f . poll (& mut Task :: new ()) . is_not_ready ()) ; let (tx , rx) = channel () ; f . map (move | r | tx . send (r) . unwrap ()) . forget () ; assert ! (rx . try_recv () . is_err ()) ; a . complete (2) ; assert_eq ! (rx . recv () . unwrap () , (1 , 2)) ; let (a , b) = promise :: < i32 > () ; let mut f = b . join (Ok (2)) ; assert ! (f . poll (& mut Task :: new ()) . is_not_ready ()) ; let (tx , rx) = channel () ; f . map (move | r | tx . send (r) . unwrap ()) . forget () ; assert ! (rx . try_recv () . is_err ()) ; a . complete (1) ; assert_eq ! (rx . recv () . unwrap () , (1 , 2)) ; let (a , b) = promise :: < i32 > () ; let mut f = finished (1) . join (b) ; assert ! (f . poll (& mut Task :: new ()) . is_not_ready ()) ; let (tx , rx) = channel () ; f . map_err (move | _r | tx . send (2) . unwrap ()) . forget () ; assert ! (rx . try_recv () . is_err ()) ; drop (a) ; assert_eq ! (rx . recv () . unwrap () , 2) ; let (a , b) = promise :: < i32 > () ; let mut f = b . join (Ok (2)) ; assert ! (f . poll (& mut Task :: new ()) . is_not_ready ()) ; let (tx , rx) = channel () ; f . map_err (move | _r | tx . send (1) . unwrap ()) . forget () ; assert ! (rx . try_recv () . is_err ()) ; drop (a) ; assert_eq ! (rx . recv () . unwrap () , 1) ; }
};
}
