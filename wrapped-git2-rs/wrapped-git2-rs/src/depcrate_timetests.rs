// Generated macro for tests (module)
macro_rules! Depcrate_timetests {
() => {
// Module: crate::time
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Time ; # [test] fn smoke () { assert_eq ! (Time :: new (1608839587 , - 300) . seconds () , 1608839587) ; assert_eq ! (Time :: new (1608839587 , - 300) . offset_minutes () , - 300) ; assert_eq ! (Time :: new (1608839587 , - 300) . sign () , '-') ; assert_eq ! (Time :: new (1608839587 , 300) . sign () , '+') ; } }
};
}
