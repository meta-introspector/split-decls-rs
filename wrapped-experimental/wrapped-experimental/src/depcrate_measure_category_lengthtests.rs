// Generated macro for tests (module)
macro_rules! Depcrate_measure_category_lengthtests {
() => {
// Module: crate::measure::category::length
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_length_category () { let meter = Length :: meter () ; let meter_parsed = MeasureUnit :: try_from_str ("meter") . unwrap () ; assert_eq ! (meter . unit , meter_parsed) ; } }
};
}
