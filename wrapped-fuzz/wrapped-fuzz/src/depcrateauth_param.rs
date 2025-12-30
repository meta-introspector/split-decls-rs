// Generated macro for auth_param (function)
macro_rules! Depcrateauth_param {
() => {
// Module: crate
// Provides: {"auth_param"}
// Dependencies: {}
# [doc = " Parses `auth-param` as in [RFC 7235 section"] # [doc = " 2.1](https://datatracker.ietf.org/doc/html/rfc7235#section-2.1)."] # [doc = ""] # [doc = " ```text"] # [doc = "   auth-param = token BWS \"=\" BWS ( token / quoted-string )"] # [doc = " ```"] fn auth_param (input : & str) -> nom :: IResult < & str , (& str , ParamValue < '_ >) > { trace ! ("auth_param attempt on {:?}" , input) ; separated_pair (token , tuple ((bws , char ('=') , bws)) , alt ((map (token , | raw | ParamValue :: new (0 , raw)) , quoted_string)) ,) (input) }
};
}
