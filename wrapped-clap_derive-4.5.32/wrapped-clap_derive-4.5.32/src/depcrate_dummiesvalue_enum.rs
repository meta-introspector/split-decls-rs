// Generated macro for value_enum (function)
macro_rules! Depcrate_dummiesvalue_enum {
() => {
// Module: crate::dummies
// Provides: {"value_enum"}
// Dependencies: {}
# [must_use] pub (crate) fn value_enum (name : & Ident) -> proc_macro2 :: TokenStream { quote ! { # [automatically_derived] impl clap :: ValueEnum for # name { fn value_variants <'a > () -> &'a [Self] { unimplemented ! () } fn from_str (_input : & str , _ignore_case : bool) -> :: std :: result :: Result < Self , String > { unimplemented ! () } fn to_possible_value <'a > (& self) -> :: std :: option :: Option < clap :: builder :: PossibleValue > { unimplemented ! () } } } }
};
}
