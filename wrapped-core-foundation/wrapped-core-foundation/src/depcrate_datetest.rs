// Generated macro for test (module)
macro_rules! Depcrate_datetest {
() => {
// Module: crate::date
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: CFDate ; use std :: cmp :: Ordering ; # [test] fn date_comparison () { let now = CFDate :: now () ; let past = CFDate :: new (now . abs_time () - 1.0) ; assert_eq ! (now . cmp (& past) , Ordering :: Greater) ; assert_eq ! (now . cmp (& now) , Ordering :: Equal) ; assert_eq ! (past . cmp (& now) , Ordering :: Less) ; } # [test] fn date_equality () { let now = CFDate :: now () ; let same_time = CFDate :: new (now . abs_time ()) ; assert_eq ! (now , same_time) ; } }
};
}
