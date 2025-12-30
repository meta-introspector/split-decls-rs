// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_inflation_basic () { let inflation = Inflation :: default () ; let mut last = inflation . total (0.0) ; for year in & [0.1 , 0.5 , 1.0 , DEFAULT_FOUNDATION_TERM , 100.0] { let total = inflation . total (* year) ; assert_eq ! (total , inflation . validator (* year) + inflation . foundation (* year)) ; assert ! (total < last) ; assert ! (total >= inflation . terminal) ; last = total ; } assert_eq ! (last , inflation . terminal) ; } # [test] fn test_inflation_fixed () { let inflation = Inflation :: new_fixed (0.001) ; for year in & [0.1 , 0.5 , 1.0 , DEFAULT_FOUNDATION_TERM , 100.0] { assert_eq ! (inflation . total (* year) , 0.001) ; } } }
};
}
