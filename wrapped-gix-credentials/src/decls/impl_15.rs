macro_rules! deps {
    () => {
        Cascade!();
        Program!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Builder"] impl Cascade { # [doc = " Extend the list of programs to run `programs`."] pub fn extend (mut self , programs : impl IntoIterator < Item = Program >) -> Self { self . programs . extend (programs) ; self } # [doc = " If `toggle` is true, http(s) urls will use the path portions of the url to obtain a credential for."] # [doc = ""] # [doc = " Otherwise, they will only take the user name into account."] pub fn use_http_path (mut self , toggle : bool) -> Self { self . use_http_path = toggle ; self } # [doc = " If `toggle` is true, a bogus password will be provided to prevent any helper program from prompting for it, nor will"] # [doc = " we prompt for the password. The resulting identity will have a bogus password and it's expected to not be used by the"] # [doc = " consuming transport."] pub fn query_user_only (mut self , toggle : bool) -> Self { self . query_user_only = toggle ; self } }
    };
}

impl_15!();