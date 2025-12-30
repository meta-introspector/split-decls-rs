// Generated macro for StateConverter (struct)
macro_rules! DepcrateStateConverter {
() => {
// Module: crate
// Provides: {"StateConverter"}
// Dependencies: {}
# [doc = " Holds information about parsing before converting into a case."] # [doc = ""] # [doc = " This struct is used when invoking the `from_case` and `with_boundaries` methods on"] # [doc = " `Casing`.  For a more fine grained approach to case conversion, consider using the [`Converter`]"] # [doc = " struct."] # [doc = " ```"] # [doc = " use convert_case::{Case, Casing};"] # [doc = ""] # [doc = " let title = \"ninety-nine_problems\".from_case(Case::Snake).to_case(Case::Title);"] # [doc = " assert_eq!(\"Ninety-nine Problems\", title);"] # [doc = " ```"] pub struct StateConverter < 'a , T : AsRef < str > > { s : & 'a T , conv : Converter , }
};
}
