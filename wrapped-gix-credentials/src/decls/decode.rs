macro_rules! deps {
    () => {
        Result!();
        Error!();
        Context!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        # [doc = ""] pub mod decode { use bstr :: { BString , ByteSlice } ; use crate :: protocol :: { context , context :: serde :: validate , Context } ; # [doc = " The error returned by [`from_bytes()`][Context::from_bytes()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Illformed UTF-8 in value of key {key:?}: {value:?}")] IllformedUtf8InValue { key : String , value : BString } , # [error (transparent)] Encoding (# [from] context :: Error) , # [error ("Invalid format in line {line:?}, expecting key=value")] Syntax { line : BString } , } impl Context { # [doc = " Decode ourselves from `input` which is the format written by [`write_to()`][Self::write_to()]."] pub fn from_bytes (input : & [u8]) -> Result < Self , Error > { let mut ctx = Context :: default () ; let Context { protocol , host , path , username , password , oauth_refresh_token , password_expiry_utc , url , quit , } = & mut ctx ; for res in input . lines () . take_while (| line | ! line . is_empty ()) . map (| line | { let mut it = line . splitn (2 , | b | * b == b'=') ; match (it . next () . and_then (| k | k . to_str () . ok ()) , it . next () . map (ByteSlice :: as_bstr) ,) { (Some (key) , Some (value)) => validate (key , value) . map (| _ | (key , value . to_owned ())) . map_err (Into :: into) , _ => Err (Error :: Syntax { line : line . into () }) , } }) { let (key , value) = res ? ; match key { "protocol" | "host" | "username" | "password" | "oauth_refresh_token" => { if ! value . is_utf8 () { return Err (Error :: IllformedUtf8InValue { key : key . into () , value }) ; } let value = value . to_string () ; * match key { "protocol" => & mut * protocol , "host" => host , "username" => username , "password" => password , "oauth_refresh_token" => oauth_refresh_token , _ => unreachable ! ("checked field names in match above") , } = Some (value) ; } "password_expiry_utc" => { * password_expiry_utc = value . to_str () . ok () . and_then (| value | value . parse () . ok ()) ; } "url" => * url = Some (value) , "path" => * path = Some (value) , "quit" => { * quit = gix_config_value :: Boolean :: try_from (value . as_ref ()) . ok () . map (Into :: into) ; } _ => { } } } Ok (ctx) } } }
    };
}

decode!();