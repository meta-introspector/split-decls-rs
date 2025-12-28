macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! AuthenticateFn {
    () => {
        deps!();
        # [doc = " A function that performs a given credential action, trying to obtain credentials for an operation that needs it."] # [doc = ""] # [doc = " Useful for both `fetch` and `push`."] # [cfg (feature = "handshake")] pub type AuthenticateFn < 'a > = Box < dyn FnMut (gix_credentials :: helper :: Action) -> gix_credentials :: protocol :: Result + 'a > ;
    };
}

AuthenticateFn!();