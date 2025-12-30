// Generated macro for time (module)
macro_rules! Depcrate_config_tree_keystime {
() => {
// Module: crate::config::tree::keys
// Provides: {"time"}
// Dependencies: {}
mod time { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , ByteSlice } , config :: tree :: { keys :: { validate , Time } , Section , } , } ; impl Time { # [doc = " Create a new instance."] pub const fn new_time (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Time) } # [doc = " Convert the `value` into a date if possible, with `now` as reference time for relative dates."] pub fn try_into_time (& self , value : Cow < '_ , BStr > , now : Option < std :: time :: SystemTime > ,) -> Result < gix_date :: Time , gix_date :: parse :: Error > { gix_date :: parse (value . as_ref () . to_str () . map_err (| _ | gix_date :: parse :: Error :: InvalidDateString { input : value . to_string () , }) ? , now ,) } } }
};
}
