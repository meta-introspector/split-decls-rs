// Generated macro for algorithm (module)
macro_rules! Depcrate_config_tree_sections_diffalgorithm {
() => {
// Module: crate::config::tree::sections::diff
// Provides: {"algorithm"}
// Dependencies: {}
mod algorithm { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: { diff :: algorithm , key , tree :: sections :: diff :: { Algorithm , Ignore } , } , } ; impl Ignore { # [doc = " See if `value` is an actual ignore"] pub fn try_into_ignore (& 'static self , value : Cow < '_ , BStr > ,) -> Result < gix_submodule :: config :: Ignore , key :: GenericErrorWithValue > { gix_submodule :: config :: Ignore :: try_from (value . as_ref ()) . map_err (| () | key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) } } impl Algorithm { # [doc = " Derive the diff algorithm identified by `name`, case-insensitively."] pub fn try_into_algorithm (& self , name : Cow < '_ , BStr >) -> Result < gix_diff :: blob :: Algorithm , algorithm :: Error > { let algo = if name . eq_ignore_ascii_case (b"myers") || name . eq_ignore_ascii_case (b"default") { gix_diff :: blob :: Algorithm :: Myers } else if name . eq_ignore_ascii_case (b"minimal") { gix_diff :: blob :: Algorithm :: MyersMinimal } else if name . eq_ignore_ascii_case (b"histogram") { gix_diff :: blob :: Algorithm :: Histogram } else if name . eq_ignore_ascii_case (b"patience") { return Err (config :: diff :: algorithm :: Error :: Unimplemented { name : name . into_owned () , }) ; } else { return Err (algorithm :: Error :: Unknown { name : name . into_owned () , }) ; } ; Ok (algo) } } }
};
}
