// Generated macro for tests (module)
macro_rules! Depcrate_measure_category_volumetests {
() => {
// Module: crate::measure::category::volume
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_volume_category () { let cubic_meter = Volume :: cubic_meter () ; let cubic_meter_parsed = MeasureUnit :: try_from_str ("cubic-meter") . unwrap () ; assert_eq ! (cubic_meter . unit , cubic_meter_parsed) ; let liter = Volume :: liter () ; let liter_parsed = MeasureUnit :: try_from_str ("liter") . unwrap () ; assert_eq ! (liter . unit , liter_parsed) ; } }
};
}
