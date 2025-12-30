// Generated macro for Value (struct)
macro_rules! Depcrate_extensions_transform_valueValue {
() => {
// Module: crate::extensions::transform::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A value used in a list of [`Fields`](super::Fields)."] # [doc = ""] # [doc = " The value has to be a sequence of one or more alphanumerical strings"] # [doc = " separated by `-`."] # [doc = " Each part of the sequence has to be no shorter than three characters and no"] # [doc = " longer than 8."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::transform::Value;"] # [doc = ""] # [doc = " \"hybrid\".parse::<Value>().expect(\"Valid Value.\");"] # [doc = ""] # [doc = " \"hybrid-foobar\".parse::<Value>().expect(\"Valid Value.\");"] # [doc = ""] # [doc = " \"no\".parse::<Value>().expect_err(\"Invalid Value.\");"] # [doc = " ```"] # [derive (Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord , Default)] pub struct Value (ShortBoxSlice < Subtag >) ;
};
}
