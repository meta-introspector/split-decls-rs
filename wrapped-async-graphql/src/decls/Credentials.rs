macro_rules! Credentials {
    () => {
        # [doc = " Indicates whether the user agent should send or receive user credentials"] # [doc = " (cookies, basic http auth, etc.) from the other domain in the case of"] # [doc = " cross-origin requests."] # [derive (Debug , Serialize , Default)] # [serde (rename_all = "kebab-case")] pub enum Credentials { # [doc = " Send user credentials if the URL is on the same origin as the calling"] # [doc = " script. This is the default value."] # [default] SameOrigin , # [doc = " Always send user credentials, even for cross-origin calls."] Include , # [doc = " Never send or receive user credentials."] Omit , }
    };
}

Credentials!()