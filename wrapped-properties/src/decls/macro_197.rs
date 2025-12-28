macro_rules! macro_197 {
    () => {
        make_binary_property ! { name : "Noncharacter_Code_Point" ; short_name : "NChar" ; ident : NoncharacterCodePoint ; data_marker : crate :: provider :: PropertyBinaryNoncharacterCodePointV1 ; singleton : SINGLETON_PROPERTY_BINARY_NONCHARACTER_CODE_POINT_V1 ; # [doc = " Code points permanently reserved for internal use."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::NoncharacterCodePoint;"] # [doc = ""] # [doc = " let noncharacter_code_point = CodePointSetData::new::<NoncharacterCodePoint>();"] # [doc = ""] # [doc = " assert!(noncharacter_code_point.contains('\\u{FDD0}'));"] # [doc = " assert!(noncharacter_code_point.contains('\\u{FFFF}'));"] # [doc = " assert!(!noncharacter_code_point.contains('\\u{10000}'));"] # [doc = " ```"] }
    };
}

macro_197!()