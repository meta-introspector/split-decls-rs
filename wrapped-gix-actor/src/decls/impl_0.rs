macro_rules! deps {
    () => {
        IdentityRef!();
        Identity!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        impl < 'a > IdentityRef < 'a > { # [doc = " Deserialize an identity from the given `data`."] pub fn from_bytes < E > (mut data : & 'a [u8]) -> Result < Self , winnow :: error :: ErrMode < E > > where E : winnow :: error :: ParserError < & 'a [u8] > + winnow :: error :: AddContext < & 'a [u8] , StrContext > , { decode :: identity . parse_next (& mut data) } # [doc = " Create an owned instance from this shared one."] pub fn to_owned (& self) -> Identity { Identity { name : self . name . to_owned () , email : self . email . to_owned () , } } # [doc = " Trim whitespace surrounding the name and email and return a new identity."] pub fn trim (& self) -> IdentityRef < 'a > { IdentityRef { name : self . name . trim () . as_bstr () , email : self . email . trim () . as_bstr () , } } }
    };
}

impl_0!();