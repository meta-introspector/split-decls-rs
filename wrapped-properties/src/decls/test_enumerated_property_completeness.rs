macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        WordBreak!();
        PropertyNamesLong!();
        VerticalOrientation!();
        IndicSyllabicCategory!();
        JoiningType!();
        HangulSyllableType!();
        SentenceBreak!();
        Baked!();
        EastAsianWidth!();
        CanonicalCombiningClass!();
        BidiClass!();
    };
}

macro_rules! test_enumerated_property_completeness {
    () => {
        deps!();
        # [cfg (test)] mod test_enumerated_property_completeness { use super :: * ; use std :: collections :: BTreeMap ; fn check_enum < 'a , T : NamedEnumeratedProperty > (lookup : & crate :: provider :: names :: PropertyValueNameToEnumMap < 'static > , consts : impl IntoIterator < Item = & 'a T > ,) where u16 : From < T > , { let mut data : BTreeMap < _ , _ > = lookup . map . iter () . map (| (name , value) | (value , (name , "Data"))) . collect () ; let names = crate :: PropertyNamesLong :: < T > :: new () ; let consts = consts . into_iter () . map (| value | { (u16 :: from (* value) as usize , (names . get (* value) . unwrap_or ("<unknown>") . to_string () , "Consts" ,) ,) }) ; let mut diff = Vec :: new () ; for t @ (value , _) in consts { if data . remove (& value) . is_none () { diff . push (t) ; } } diff . extend (data) ; let mut fmt_diff = String :: new () ; for (value , (name , source)) in diff { fmt_diff . push_str (& format ! ("{source}:\t{name} = {value:?}\n")) ; } assert ! (fmt_diff . is_empty () , "Values defined in data do not match values defined in consts. Difference:\n{fmt_diff}") ; } # [test] fn test_ea () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_EAST_ASIAN_WIDTH_V1 , EastAsianWidth :: ALL_VALUES ,) ; } # [test] fn test_ccc () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_CANONICAL_COMBINING_CLASS_V1 , CanonicalCombiningClass :: ALL_VALUES ,) ; } # [test] fn test_jt () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_JOINING_TYPE_V1 , JoiningType :: ALL_VALUES ,) ; } # [test] fn test_insc () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_INDIC_SYLLABIC_CATEGORY_V1 , IndicSyllabicCategory :: ALL_VALUES ,) ; } # [test] fn test_sb () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_SENTENCE_BREAK_V1 , SentenceBreak :: ALL_VALUES ,) ; } # [test] fn test_wb () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_WORD_BREAK_V1 , WordBreak :: ALL_VALUES ,) ; } # [test] fn test_bc () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_BIDI_CLASS_V1 , BidiClass :: ALL_VALUES ,) ; } # [test] fn test_hst () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_HANGUL_SYLLABLE_TYPE_V1 , HangulSyllableType :: ALL_VALUES ,) ; } # [test] fn test_vo () { check_enum (crate :: provider :: Baked :: SINGLETON_PROPERTY_NAME_PARSE_VERTICAL_ORIENTATION_V1 , VerticalOrientation :: ALL_VALUES ,) ; } }
    };
}

test_enumerated_property_completeness!()