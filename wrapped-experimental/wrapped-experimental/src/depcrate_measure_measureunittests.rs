// Generated macro for tests (module)
macro_rules! Depcrate_measure_measureunittests {
() => {
// Module: crate::measure::measureunit
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_generate_short_representation () { let test_cases = vec ! [("meter" , "I85") , ("foot" , "I50") , ("inch" , "I66") , ("square-meter" , "P2I85") , ("square-millimeter" , "P2D-3I85") , ("micrometer" , "D-6I85") , ("meter-per-second" , "I85P-1I127") , ("liter-per-100-kilometer" , "C100I82P-1D3I85") , ("portion-per-1e9" , "C1E9I113") , ("per-10000000000-portion" , "C1E10P-1I113") , ("liter-per-240000000000-kilometer" , "C24E10I82P-1D3I85") , ("millimeter-per-square-microsecond" , "D-3I85P-2D-6I127") ,] ; for (full_unit , expected_short) in test_cases { let measure_unit = MeasureUnit :: try_from_str (full_unit) . unwrap () ; let short_representation = measure_unit . generate_short_representation () ; assert_eq ! (short_representation , expected_short , "{full_unit}") ; } } }
};
}
