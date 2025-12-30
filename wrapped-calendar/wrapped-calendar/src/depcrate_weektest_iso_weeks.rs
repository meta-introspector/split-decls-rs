// Generated macro for test_iso_weeks (function)
macro_rules! Depcrate_weektest_iso_weeks {
() => {
// Module: crate::week
// Provides: {"test_iso_weeks"}
// Dependencies: {}
# [test] fn test_iso_weeks () { use crate :: types :: IsoWeekOfYear ; use crate :: Date ; # [expect (clippy :: zero_prefixed_literal)] for ((y , m , d) , (iso_year , week_number)) in [((2009 , 12 , 30) , (2009 , 53)) , ((2009 , 12 , 31) , (2009 , 53)) , ((2010 , 01 , 01) , (2009 , 53)) , ((2010 , 01 , 02) , (2009 , 53)) , ((2010 , 01 , 03) , (2009 , 53)) , ((2010 , 01 , 04) , (2010 , 1)) , ((2010 , 01 , 05) , (2010 , 1)) , ((2029 , 12 , 29) , (2029 , 52)) , ((2029 , 12 , 30) , (2029 , 52)) , ((2029 , 12 , 31) , (2030 , 1)) , ((2030 , 01 , 01) , (2030 , 1)) , ((2030 , 01 , 02) , (2030 , 1)) , ((2030 , 01 , 03) , (2030 , 1)) , ((2030 , 01 , 04) , (2030 , 1)) ,] { assert_eq ! (Date :: try_new_iso (y , m , d) . unwrap () . week_of_year () , IsoWeekOfYear { iso_year , week_number }) ; } }
};
}
