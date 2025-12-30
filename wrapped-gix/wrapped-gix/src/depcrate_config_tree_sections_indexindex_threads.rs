// Generated macro for index_threads (module)
macro_rules! Depcrate_config_tree_sections_indexindex_threads {
() => {
// Module: crate::config::tree::sections::index
// Provides: {"index_threads"}
// Dependencies: {}
mod index_threads { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: { key :: GenericErrorWithValue , tree :: index :: IndexThreads } , } ; impl IndexThreads { # [doc = " Parse `value` into the amount of threads to use, with `1` being single-threaded, or `0` indicating"] # [doc = " to select the amount of threads, with any other number being the specific amount of threads to use."] pub fn try_into_index_threads (& 'static self , value : Cow < '_ , BStr > ,) -> Result < usize , config :: key :: GenericErrorWithValue > { gix_config :: Integer :: try_from (value . as_ref ()) . ok () . and_then (| i | i . to_decimal () . and_then (| i | i . try_into () . ok ())) . or_else (| | { gix_config :: Boolean :: try_from (value . as_ref ()) . ok () . map (| b | if b . 0 { 0 } else { 1 }) }) . ok_or_else (| | GenericErrorWithValue :: from_value (self , value . into_owned ())) } } }
};
}
