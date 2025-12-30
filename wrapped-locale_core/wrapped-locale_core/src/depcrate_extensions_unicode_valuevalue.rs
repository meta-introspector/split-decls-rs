// Generated macro for Value (struct)
macro_rules! Depcrate_extensions_unicode_valueValue {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A value used in a list of [`Keywords`](super::Keywords)."] # [doc = ""] # [doc = " The value has to be a sequence of one or more alphanumerical strings"] # [doc = " separated by `-`."] # [doc = " Each part of the sequence has to be no shorter than three characters and no"] # [doc = " longer than 8."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::{value, Value};"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " assert_writeable_eq!(value!(\"gregory\"), \"gregory\");"] # [doc = " assert_writeable_eq!("] # [doc = "     \"islamic-civil\".parse::<Value>().unwrap(),"] # [doc = "     \"islamic-civil\""] # [doc = " );"] # [doc = ""] # [doc = " // The value \"true\" has the special, empty string representation"] # [doc = " assert_eq!(value!(\"true\").to_string(), \"\");"] # [doc = " ```"] # [derive (Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord , Default)] pub struct Value (ShortBoxSlice < Subtag >) ;
};
}
