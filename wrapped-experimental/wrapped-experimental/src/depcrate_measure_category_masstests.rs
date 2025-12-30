// Generated macro for tests (module)
macro_rules! Depcrate_measure_category_masstests {
() => {
// Module: crate::measure::category::mass
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_mass_category () { let gram = Mass :: gram () ; let gram_parsed = MeasureUnit :: try_from_str ("gram") . unwrap () ; assert_eq ! (gram . unit , gram_parsed) ; let kilogram = Mass :: kilogram () ; let kilogram_parsed = MeasureUnit :: try_from_str ("kilogram") . unwrap () ; assert_eq ! (kilogram . unit , kilogram_parsed) ; } }
};
}
