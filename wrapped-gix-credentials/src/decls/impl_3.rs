macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Outcome { # [doc = " Try to fetch username _and_ password to form an identity. This will fail if one of them is not set."] # [doc = ""] # [doc = " This does nothing if only one of the fields is set, or consume both."] pub fn consume_identity (& mut self) -> Option < gix_sec :: identity :: Account > { if self . username . is_none () || self . password . is_none () { return None ; } self . username . take () . zip (self . password . take ()) . map (| (username , password) | gix_sec :: identity :: Account { username , password , oauth_refresh_token : self . oauth_refresh_token . take () , }) } }
    };
}

impl_3!();