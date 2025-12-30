// Generated macro for tests (module)
macro_rules! Depcrate_measure_category_areatests {
() => {
// Module: crate::measure::category::area
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_area_category () { let square_meter = Area :: square_meter () ; let square_meter_parsed = MeasureUnit :: try_from_str ("square-meter") . unwrap () ; assert_eq ! (square_meter . unit , square_meter_parsed) ; } }
};
}
