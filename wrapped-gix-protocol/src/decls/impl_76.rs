macro_rules! deps {
    () => {
        RemoteProgress!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl RemoteProgress < '_ > { # [doc = " Parse the progress from a typical git progress `line` as sent by the remote."] pub fn from_bytes (mut line : & [u8]) -> Option < RemoteProgress < '_ > > { parse_progress (& mut line) . ok () . and_then (| r | { if r . percent . is_none () && r . step . is_none () && r . max . is_none () { None } else { Some (r) } }) } # [doc = " Parse `text`, which is interpreted as error if `is_error` is true, as [`RemoteProgress`] and call the respective"] # [doc = " methods on the given `progress` instance."] pub fn translate_to_progress (is_error : bool , text : & [u8] , progress : & mut impl gix_features :: progress :: Progress) { fn progress_name (current : Option < String > , action : & [u8]) -> String { match current { Some (current) => format ! ("{}: {}" , current . split_once (':') . map_or (&* current , | x | x . 0) , action . as_bstr ()) , None => action . as_bstr () . to_string () , } } if is_error { if ! text . is_empty () { progress . fail (progress_name (None , text)) ; } } else { match RemoteProgress :: from_bytes (text) { Some (RemoteProgress { action , percent : _ , step , max , }) => { progress . set_name (progress_name (progress . name () , action)) ; progress . init (max , gix_features :: progress :: count ("objects")) ; if let Some (step) = step { progress . set (step) ; } } None => progress . set_name (progress_name (progress . name () , text)) , } } } }
    };
}

impl_76!();