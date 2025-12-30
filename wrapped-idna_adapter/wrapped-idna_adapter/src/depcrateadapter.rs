// Generated macro for Adapter (struct)
macro_rules! DepcrateAdapter {
() => {
// Module: crate
// Provides: {"Adapter"}
// Dependencies: {}
# [doc = " An adapter between a Unicode back end an the `idna` crate."] pub struct Adapter { mapper : Uts46MapperBorrowed < 'static > , canonical_combining_class : CanonicalCombiningClassMapBorrowed < 'static > , general_category : CodePointMapDataBorrowed < 'static , GeneralCategory > , bidi_class : CodePointMapDataBorrowed < 'static , icu_properties :: props :: BidiClass > , joining_type : CodePointMapDataBorrowed < 'static , icu_properties :: props :: JoiningType > , }
};
}
