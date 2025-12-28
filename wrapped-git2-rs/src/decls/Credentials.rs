macro_rules! deps {
    () => {
        Cred!();
        Error!();
    };
}

macro_rules! Credentials {
    () => {
        deps!();
        # [doc = " Callback used to acquire credentials for when a remote is fetched."] # [doc = ""] # [doc = " * `url` - the resource for which the credentials are required."] # [doc = " * `username_from_url` - the username that was embedded in the URL, or `None`"] # [doc = "                         if it was not included."] # [doc = " * `allowed_types` - a bitmask stating which cred types are OK to return."] pub type Credentials < 'a > = dyn FnMut (& str , Option < & str > , CredentialType) -> Result < Cred , Error > + 'a ;
    };
}

Credentials!();