// Generated macro for test_basic (function)
macro_rules! Depcrate_dimension_units_formattest_basic {
() => {
// Module: crate::dimension::units::format
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu_locale_core :: locale ; use writeable :: assert_writeable_eq ; use crate :: dimension :: units :: formatter :: UnitsFormatter ; use crate :: dimension :: units :: options :: { UnitsFormatterOptions , Width } ; let test_cases = [(locale ! ("en-US") , "meter" , "1" , UnitsFormatterOptions { width : Width :: Long , .. Default :: default () } , "1 meter" ,) , (locale ! ("en-US") , "meter" , "12345.67" , UnitsFormatterOptions :: default () , "12,345.67 m" ,) , (locale ! ("en-US") , "century" , "12345.67" , UnitsFormatterOptions { width : Width :: Long , .. Default :: default () } , "12,345.67 centuries" ,) , (locale ! ("de-DE") , "meter" , "12345.67" , UnitsFormatterOptions :: default () , "12.345,67 m" ,) , (locale ! ("ar-EG") , "meter" , "12345.67" , UnitsFormatterOptions { width : Width :: Long , .. Default :: default () } , "١٢٬٣٤٥٫٦٧ متر" ,) ,] ; for (locale , unit , value , options , expected) in test_cases { let fmt = UnitsFormatter :: try_new (locale . into () , unit , options) . unwrap () ; let value = value . parse () . unwrap () ; assert_writeable_eq ! (fmt . format_fixed_decimal (& value) , expected) ; } }
};
}
