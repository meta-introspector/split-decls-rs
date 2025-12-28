macro_rules! macro_194 {
    () => {
        make_binary_property ! { name : "Lowercase" ; short_name : "Lower" ; ident : Lowercase ; data_marker : crate :: provider :: PropertyBinaryLowercaseV1 ; singleton : SINGLETON_PROPERTY_BINARY_LOWERCASE_V1 ; # [doc = " Lowercase characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Lowercase;"] # [doc = ""] # [doc = " let lowercase = CodePointSetData::new::<Lowercase>();"] # [doc = ""] # [doc = " assert!(lowercase.contains('a'));"] # [doc = " assert!(!lowercase.contains('A'));"] # [doc = " ```"] }
    };
}

macro_194!();