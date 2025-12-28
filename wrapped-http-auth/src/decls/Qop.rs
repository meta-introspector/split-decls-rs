macro_rules! Qop {
    () => {
        # [doc = " \"Quality of protection\" value."] # [doc = ""] # [doc = " The values here can be used in a bitmask as in [`DigestClient::qop`]."] # [derive (Copy , Clone , Debug)] # [repr (u8)] # [non_exhaustive] pub enum Qop { # [doc = " Authentication."] Auth = 1 , # [doc = " Authentication with integrity protection."] # [doc = ""] # [doc = " \"Integrity protection\" means protection of the request entity body."] AuthInt = 2 , }
    };
}

Qop!();