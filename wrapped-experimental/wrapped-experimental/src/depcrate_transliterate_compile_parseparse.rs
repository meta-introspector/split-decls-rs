// Generated macro for parse (function)
macro_rules! Depcrate_transliterate_compile_parseparse {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"parse"}
// Dependencies: {}
# [cfg (test)] pub (super) fn parse (source : & str) -> Result < Vec < Rule > > { use icu :: properties :: CodePointSetData ; Parser :: run (source , & CodePointSetData :: new :: < XidStart > () . static_to_owned () . to_code_point_inversion_list () , & CodePointSetData :: new :: < XidContinue > () . static_to_owned () . to_code_point_inversion_list () , & CodePointSetData :: new :: < PatternWhiteSpace > () . static_to_owned () . to_code_point_inversion_list () , & icu_properties :: provider :: Baked ,) }
};
}
