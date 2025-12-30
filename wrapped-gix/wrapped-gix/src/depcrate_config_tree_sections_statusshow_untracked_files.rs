// Generated macro for show_untracked_files (module)
macro_rules! Depcrate_config_tree_sections_statusshow_untracked_files {
() => {
// Module: crate::config::tree::sections::status
// Provides: {"show_untracked_files"}
// Dependencies: {}
mod show_untracked_files { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: status :: ShowUntrackedFiles , status } ; impl ShowUntrackedFiles { pub fn try_into_show_untracked_files (& 'static self , value : Cow < '_ , BStr > ,) -> Result < status :: UntrackedFiles , config :: key :: GenericErrorWithValue > { use crate :: bstr :: ByteSlice ; Ok (match value . as_ref () . as_bytes () { b"no" => status :: UntrackedFiles :: None , b"normal" => status :: UntrackedFiles :: Collapsed , b"all" => status :: UntrackedFiles :: Files , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , }) } } }
};
}
