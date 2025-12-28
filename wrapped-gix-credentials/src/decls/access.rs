macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! access {
    () => {
        deps!();
        mod access { use bstr :: BString ; use crate :: protocol :: Context ; impl Context { # [doc = " Clear all fields that are considered secret."] pub fn clear_secrets (& mut self) { let Context { protocol : _ , host : _ , path : _ , username : _ , password , oauth_refresh_token , password_expiry_utc : _ , url : _ , quit : _ , } = self ; * password = None ; * oauth_refresh_token = None ; } # [doc = " Replace existing secrets with the word `<redacted>`."] pub fn redacted (mut self) -> Self { let Context { protocol : _ , host : _ , path : _ , username : _ , password , oauth_refresh_token , password_expiry_utc : _ , url : _ , quit : _ , } = & mut self ; for secret in [password , oauth_refresh_token] . into_iter () . flatten () { * secret = "<redacted>" . into () ; } self } # [doc = " Convert all relevant fields into a URL for consumption."] pub fn to_url (& self) -> Option < BString > { use bstr :: { ByteSlice , ByteVec } ; let mut buf : BString = self . protocol . clone () ? . into () ; buf . push_str (b"://") ; if let Some (user) = & self . username { buf . push_str (user) ; buf . push (b'@') ; } if let Some (host) = & self . host { buf . push_str (host) ; } if let Some (path) = & self . path { if ! path . starts_with_str ("/") { buf . push (b'/') ; } buf . push_str (path) ; } buf . into () } # [doc = " Compute a prompt to obtain the given value."] pub fn to_prompt (& self , field : & str) -> String { match self . to_url () { Some (url) => format ! ("{field} for {url}: ") , None => format ! ("{field}: ") , } } } }
    };
}

access!();