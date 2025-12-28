macro_rules! deps {
    () => {
        PropertyCodePointSet!();
        CodePointSetDataBorrowed!();
    };
}

macro_rules! CodePointSetData {
    () => {
        deps!();
        # [doc = " A set of Unicode code points. Access its data via the borrowed version,"] # [doc = " [`CodePointSetDataBorrowed`]."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Alphabetic;"] # [doc = ""] # [doc = " let alphabetic = CodePointSetData::new::<Alphabetic>();"] # [doc = ""] # [doc = " assert!(!alphabetic.contains('3'));"] # [doc = " assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE"] # [doc = " assert!(alphabetic.contains('A'));"] # [doc = " assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS"] # [doc = " ```"] # [derive (Debug)] pub struct CodePointSetData { data : DataPayload < ErasedMarker < PropertyCodePointSet < 'static > > > , }
    };
}

CodePointSetData!();