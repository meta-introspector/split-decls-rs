macro_rules! macro_152 {
    () => {
        make_binary_property ! { name : "ASCII_Hex_Digit" ; short_name : "AHex" ; ident : AsciiHexDigit ; data_marker : crate :: provider :: PropertyBinaryAsciiHexDigitV1 ; singleton : SINGLETON_PROPERTY_BINARY_ASCII_HEX_DIGIT_V1 ; # [doc = " ASCII characters commonly used for the representation of hexadecimal numbers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::AsciiHexDigit;"] # [doc = ""] # [doc = " let ascii_hex_digit = CodePointSetData::new::<AsciiHexDigit>();"] # [doc = ""] # [doc = " assert!(ascii_hex_digit.contains('3'));"] # [doc = " assert!(!ascii_hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE"] # [doc = " assert!(ascii_hex_digit.contains('A'));"] # [doc = " assert!(!ascii_hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS"] # [doc = " ```"] }
    };
}

macro_152!();