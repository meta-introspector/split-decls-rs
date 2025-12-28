macro_rules! macro_195 {
    () => {
        make_binary_property ! { name : "Math" ; short_name : "Math" ; ident : Math ; data_marker : crate :: provider :: PropertyBinaryMathV1 ; singleton : SINGLETON_PROPERTY_BINARY_MATH_V1 ; # [doc = " Characters used in mathematical notation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Math;"] # [doc = ""] # [doc = " let math = CodePointSetData::new::<Math>();"] # [doc = ""] # [doc = " assert!(math.contains('='));"] # [doc = " assert!(math.contains('+'));"] # [doc = " assert!(!math.contains('-'));"] # [doc = " assert!(math.contains('−'));  // U+2212 MINUS SIGN"] # [doc = " assert!(!math.contains('/'));"] # [doc = " assert!(math.contains('∕'));  // U+2215 DIVISION SLASH"] # [doc = " ```"] }
    };
}

macro_195!();