macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        BidiMirroringGlyph!();
    };
}

macro_rules! macro_361 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'BidiMirroringGlyph' Unicode property"] PropertyEnumBidiMirroringGlyphV1 , PropertyCodePointMap <'static , crate :: bidi :: BidiMirroringGlyph >, is_singleton = true ,) ;
    };
}

macro_361!()