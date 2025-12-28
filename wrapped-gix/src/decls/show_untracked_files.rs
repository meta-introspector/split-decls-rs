macro_rules! deps {
    () => {
        UntrackedFiles!();
        ShowUntrackedFiles!();
    };
}

macro_rules! show_untracked_files {
    () => {
        deps!();
        mod show_untracked_files { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: status :: ShowUntrackedFiles , status } ; impl ShowUntrackedFiles { pub fn try_into_show_untracked_files (& 'static self , value : Cow < '_ , BStr > ,) -> Result < status :: UntrackedFiles , config :: key :: GenericErrorWithValue > { use crate :: bstr :: ByteSlice ; Ok (match value . as_ref () . as_bytes () { b"no" => status :: UntrackedFiles :: None , b"normal" => status :: UntrackedFiles :: Collapsed , b"all" => status :: UntrackedFiles :: Files , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , }) } } }
    };
}

show_untracked_files!();