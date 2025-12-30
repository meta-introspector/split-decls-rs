// Generated macro for IterDefinedNames (struct)
macro_rules! Depcrate_tests_iterIterDefinedNames {
() => {
// Module: crate::tests::iter
// Provides: {"IterDefinedNames"}
// Dependencies: {}
# [doc = "\nAn iterator over all defined named flags.\n\nThis iterator will yield flags values for all defined named flags, regardless of\nwhether they are contained in a particular flags value.\n"] pub struct IterDefinedNames < B : 'static > { flags : & 'static [Flag < B >] , idx : usize , }
};
}
