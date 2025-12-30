// Generated macro for CaseMapper (struct)
macro_rules! Depcrate_casemapperCaseMapper {
() => {
// Module: crate::casemapper
// Provides: {"CaseMapper"}
// Dependencies: {}
# [doc = " A struct with the ability to convert characters and strings to uppercase or lowercase,"] # [doc = " or fold them to a normalized form for case-insensitive comparison."] # [doc = ""] # [doc = " Most methods for this type live on [`CaseMapperBorrowed`], which you can obtain via"] # [doc = " [`CaseMapper::new()`] or [`CaseMapper::as_borrowed()`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::CaseMapper;"] # [doc = " use icu::locale::langid;"] # [doc = ""] # [doc = " let cm = CaseMapper::new();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     cm.uppercase_to_string(\"hello world\", &langid!(\"und\")),"] # [doc = "     \"HELLO WORLD\""] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     cm.lowercase_to_string(\"Γειά σου Κόσμε\", &langid!(\"und\")),"] # [doc = "     \"γειά σου κόσμε\""] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct CaseMapper { pub (crate) data : DataPayload < CaseMapV1 > , }
};
}
