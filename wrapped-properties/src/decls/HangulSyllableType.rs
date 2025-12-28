macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! HangulSyllableType {
    () => {
        deps!();
        # [doc = " Enumerated property Hangul_Syllable_Type"] # [doc = ""] # [doc = " The Unicode standard provides both precomposed Hangul syllables and conjoining Jamo to compose"] # [doc = " arbitrary Hangul syllables. This property provides that ontology of Hangul code points."] # [doc = ""] # [doc = " For more information, see the [Unicode Korean FAQ](https://www.unicode.org/faq/korean.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::HangulSyllableType, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<HangulSyllableType>::new().get('ᄀ'),"] # [doc = "     HangulSyllableType::LeadingJamo"] # [doc = " ); // U+1100"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<HangulSyllableType>::new().get('가'),"] # [doc = "     HangulSyllableType::LeadingVowelSyllable"] # [doc = " ); // U+AC00"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct HangulSyllableType (pub (crate) u8) ;
    };
}

HangulSyllableType!()