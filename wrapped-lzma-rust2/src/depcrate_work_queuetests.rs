// Generated macro for tests (module)
macro_rules! Depcrate_work_queuetests {
() => {
// Module: crate::work_queue
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: { thread , time :: Duration } ; use super :: * ; # [test] fn test_basic_functionality () { let queue = WorkStealingQueue :: new () ; let worker = queue . worker () ; assert ! (queue . push (1)) ; assert ! (queue . push (2)) ; assert ! (queue . push (3)) ; assert_eq ! (worker . steal () , Some (1)) ; assert_eq ! (worker . steal () , Some (2)) ; assert_eq ! (worker . try_steal () , Some (3)) ; assert_eq ! (worker . try_steal () , None) ; queue . close () ; assert ! (! queue . push (4)) ; assert ! (worker . is_closed_and_empty ()) ; } # [test] fn test_multiple_workers () { let queue = WorkStealingQueue :: new () ; let worker1 = queue . worker () ; let worker2 = queue . worker () ; for i in 0 .. 10 { queue . push (i) ; } let mut results = Vec :: new () ; while let Some (item) = worker1 . try_steal () { results . push (item) ; } while let Some (item) = worker2 . try_steal () { results . push (item) ; } results . sort () ; assert_eq ! (results , (0 .. 10) . collect ::< Vec < _ >> ()) ; } # [test] fn test_blocking_behavior () { let queue = WorkStealingQueue :: new () ; let worker = queue . worker () ; let queue_clone = WorkStealingQueue { inner : Arc :: clone (& queue . inner) , } ; thread :: spawn (move | | { thread :: sleep (Duration :: from_millis (50)) ; queue_clone . push (42) ; queue_clone . close () ; }) ; assert_eq ! (worker . steal () , Some (42)) ; assert_eq ! (worker . steal () , None) ; } }
};
}
