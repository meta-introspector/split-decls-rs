macro_rules! macro_182 {
    () => {
        make_binary_property ! { name : "Hex_Digit" ; short_name : "Hex" ; ident : HexDigit ; data_marker : crate :: provider :: PropertyBinaryHexDigitV1 ; singleton : SINGLETON_PROPERTY_BINARY_HEX_DIGIT_V1 ; # [doc = " Characters commonly used for the representation of hexadecimal numbers, plus their"] # [doc = " compatibility equivalents."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::HexDigit;"] # [doc = ""] # [doc = " let hex_digit = CodePointSetData::new::<HexDigit>();"] # [doc = ""] # [doc = " assert!(hex_digit.contains('0'));"] # [doc = " assert!(!hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE"] # [doc = " assert!(hex_digit.contains('f'));"] # [doc = " assert!(hex_digit.contains('ｆ'));  // U+FF46 FULLWIDTH LATIN SMALL LETTER F"] # [doc = " assert!(hex_digit.contains('Ｆ'));  // U+FF26 FULLWIDTH LATIN CAPITAL LETTER F"] # [doc = " assert!(!hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS"] # [doc = " ```"] }
    };
}

macro_182!()