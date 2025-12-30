// Generated macro for another_test (module)
macro_rules! Depcrate_sugaranother_test {
() => {
// Module: crate::sugar
// Provides: {"another_test"}
// Dependencies: {}
# [cfg (test)] mod another_test { use crate :: sugar ; # [allow (dead_code)] fn can_access_pub_compose () { let _ = sugar :: test :: two_ints_pub (42) ; let _ = sugar :: test :: two_ints_pub_with_attrs (42) ; } }
};
}
