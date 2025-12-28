macro_rules! deps {
    () => {
        Error!();
        V2!();
        Claims!();
        TrustedToken!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl TrustedToken { pub (crate) fn _new (header : & str , payload : & [u8] , footer : & [u8] , implicit_assert : & [u8] ,) -> Result < Self , Error > { Ok (Self { header : header . to_string () , payload : String :: from_utf8 (payload . to_vec ()) . map_err (| _ | Error :: PayloadInvalidUtf8) ? , # [cfg (feature = "std")] payload_claims : None , footer : footer . to_vec () , implicit_assert : implicit_assert . to_vec () , }) } # [doc = " Get the header that is used for this token."] pub fn header (& self) -> & str { & self . header } # [doc = " Get the payload that is used for this token."] pub fn payload (& self) -> & str { & self . payload } # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [doc = " Return the optional and validated [`Claims`] parsed from the tokens payload."] # [doc = ""] # [doc = " - `None`: If no [`Claims`] have been parsed or validated."] # [doc = " - `Some`: If some [`Claims`] have been parsed **AND** validated."] # [doc = ""] # [doc = " [`Claims`]: crate::claims::Claims"] pub fn payload_claims (& self) -> Option < & Claims > { debug_assert ! (self . payload_claims . is_some ()) ; match & self . payload_claims { Some (claims) => Some (claims) , None => None , } } # [cfg (all (feature = "std" , feature = "v4"))] # [doc = " Set the payload claims **AFTER HAVING VALIDATED THEM**."] pub (crate) fn set_payload_claims (& mut self , claims : Claims) { self . payload_claims = Some (claims) ; } # [doc = " Get the footer used to create the token."] # [doc = ""] # [doc = " Empty if `None` was used during creation."] pub fn footer (& self) -> & [u8] { & self . footer } # [doc = " Get the implicit assertion used to create the token."] # [doc = ""] # [doc = " Empty if `None` was used during creation."] # [doc = " If token was created using `V2`, then it will always be empty."] pub fn implicit_assert (& self) -> & [u8] { & self . implicit_assert } }
    };
}

impl_127!();