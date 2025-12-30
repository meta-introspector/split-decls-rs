// Generated macro for DisplayHint (enum)
macro_rules! Depcrate_display_hintDisplayHint {
() => {
// Module: crate::display_hint
// Provides: {"DisplayHint"}
// Dependencies: {}
# [doc = " All display hints"] # [derive (Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum DisplayHint { NoHint { zero_pad : usize , } , # [doc = " `:x` OR `:X`"] Hexadecimal { alternate : bool , uppercase : bool , zero_pad : usize , } , # [doc = " `:o`"] Octal { alternate : bool , zero_pad : usize , } , # [doc = " `:b`"] Binary { alternate : bool , zero_pad : usize , } , # [doc = " `:a`"] Ascii , # [doc = " `:?`"] Debug , # [doc = " `:us` `:ms`, formats integers as timestamps in seconds"] Seconds (TimePrecision) , # [doc = " `:tus` `:tms` `:ts`, formats integers as human-readable time"] Time (TimePrecision) , # [doc = " `:iso8601{ms,s}`, formats integers as timestamp in ISO8601 date time format"] ISO8601 (TimePrecision) , # [doc = " `__internal_bitflags_NAME` instructs the decoder to print the flags that are set, instead of"] # [doc = " the raw value."] Bitflags { name : String , package : String , disambiguator : String , crate_name : Option < String > , } , # [doc = " `:cbor`: There is CBOR data encoded in those bytes, to be shown in diagnostic notation."] # [doc = ""] # [doc = " Technically, the byte string interpreted as a CBOR sequence, and shown in the diagnostic"] # [doc = " notation of a sequence. That is identical to processing a single CBOR item if there is just"] # [doc = " one present, but also allows reporting multiple items from consecutive memory; diagnostic"] # [doc = " notation turns those into comma separated items."] Cbor , # [doc = " Display hints currently not supported / understood"] Unknown (String) , }
};
}
