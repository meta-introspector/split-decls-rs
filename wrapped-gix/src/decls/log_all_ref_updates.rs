macro_rules! deps {
    () => {
        LogAllRefUpdates!();
        Error!();
    };
}

macro_rules! log_all_ref_updates {
    () => {
        deps!();
        mod log_all_ref_updates { use crate :: { config , config :: tree :: core :: LogAllRefUpdates } ; impl LogAllRefUpdates { # [doc = " Returns the mode for ref-updates as parsed from `value`. If `value` is not a boolean, we try"] # [doc = " to interpret the string value instead. For correctness, this two step process is necessary as"] # [doc = " the interpretation of booleans in special in `git-config`, i.e. we can't just treat it as string."] pub fn try_into_ref_updates (& 'static self , value : Option < Result < bool , gix_config :: value :: Error > > ,) -> Result < Option < gix_ref :: store :: WriteReflog > , config :: key :: GenericErrorWithValue > { match value { Some (Ok (bool)) => Ok (Some (if bool { gix_ref :: store :: WriteReflog :: Normal } else { gix_ref :: store :: WriteReflog :: Disable })) , Some (Err (err)) => match err . input { val if val . eq_ignore_ascii_case (b"always") => Ok (Some (gix_ref :: store :: WriteReflog :: Always)) , val => Err (config :: key :: GenericErrorWithValue :: from_value (self , val)) , } , None => Ok (None) , } } } }
    };
}

log_all_ref_updates!()