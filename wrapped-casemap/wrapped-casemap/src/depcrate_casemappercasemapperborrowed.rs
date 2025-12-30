// Generated macro for CaseMapperBorrowed (struct)
macro_rules! Depcrate_casemapperCaseMapperBorrowed {
() => {
// Module: crate::casemapper
// Provides: {"CaseMapperBorrowed"}
// Dependencies: {}
# [doc = " A struct with the ability to convert characters and strings to uppercase or lowercase,"] # [doc = " or fold them to a normalized form for case-insensitive comparison, borrowed version."] # [doc = ""] # [doc = " See methods or [`CaseMapper`] for examples."] # [derive (Clone , Debug , Copy)] pub struct CaseMapperBorrowed < 'a > { pub (crate) data : & 'a CaseMap < 'a > , }
};
}
