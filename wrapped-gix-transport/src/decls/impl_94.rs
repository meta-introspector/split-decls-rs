macro_rules! deps {
    () => {
        Service!();
        Http!();
        Error!();
        Transport!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < H : Http > Transport < H > { fn check_content_type (service : Service , kind : & str , headers : < H as Http > :: Headers) -> Result < () , client :: Error > { let wanted_content_type = format ! ("application/x-{}-{}" , service . as_str () , kind) ; if ! headers . lines () . collect :: < Result < Vec < _ > , _ > > () ? . iter () . any (| l | { let mut tokens = l . split (':') ; tokens . next () . zip (tokens . next ()) . is_some_and (| (name , value) | { name . eq_ignore_ascii_case ("content-type") && value . trim () == wanted_content_type }) }) { return Err (client :: Error :: Http (Error :: Detail { description : format ! ("Didn't find '{wanted_content_type}' header to indicate 'smart' protocol, and 'dumb' protocol is not supported.") , })) ; } Ok (()) } # [allow (clippy :: unnecessary_wraps , unknown_lints)] fn add_basic_auth_if_present (& self , headers : & mut Vec < Cow < '_ , str > >) -> Result < () , client :: Error > { if let Some (gix_sec :: identity :: Account { username , password , oauth_refresh_token : _ , }) = & self . identity { # [cfg (not (feature = "http-client-insecure-credentials"))] if self . url . starts_with ("http://") { return Err (client :: Error :: AuthenticationRefused ("Will not send credentials in clear text over http" ,)) ; } headers . push (Cow :: Owned (format ! ("Authorization: Basic {}" , base64 :: engine :: general_purpose :: STANDARD . encode (format ! ("{username}:{password}"))))) ; } Ok (()) } }
    };
}

impl_94!();