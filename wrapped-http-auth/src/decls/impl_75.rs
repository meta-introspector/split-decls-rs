macro_rules! deps {
    () => {
        PasswordClientBuilder!();
        PasswordClient!();
        Error!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [doc = " Tries to create a `PasswordClient` from the supplied `str` challenge list."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] impl TryFrom < & str > for PasswordClient { type Error = String ; # [inline] fn try_from (value : & str) -> Result < Self , Self :: Error > { PasswordClient :: builder () . challenges (value) . build () } }
    };
}

impl_75!()