macro_rules! macro_156 {
    () => {
        make_binary_property ! { name : "Bidi_Mirrored" ; short_name : "Bidi_M" ; ident : BidiMirrored ; data_marker : crate :: provider :: PropertyBinaryBidiMirroredV1 ; singleton : SINGLETON_PROPERTY_BINARY_BIDI_MIRRORED_V1 ; # [doc = " Characters that are mirrored in bidirectional text."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::BidiMirrored;"] # [doc = ""] # [doc = " let bidi_mirrored = CodePointSetData::new::<BidiMirrored>();"] # [doc = ""] # [doc = " assert!(bidi_mirrored.contains('['));"] # [doc = " assert!(bidi_mirrored.contains(']'));"] # [doc = " assert!(bidi_mirrored.contains('∑'));  // U+2211 N-ARY SUMMATION"] # [doc = " assert!(!bidi_mirrored.contains('ཉ'));  // U+0F49 TIBETAN LETTER NYA"] # [doc = " ```"] }
    };
}

macro_156!();