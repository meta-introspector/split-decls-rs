macro_rules! Authorization {
    () => {
        # [doc = " [`authorizer`](Connection::authorizer) return code"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Authorization { # [doc = " Authorize the action."] Allow , # [doc = " Don't allow access, but don't trigger an error either."] Ignore , # [doc = " Trigger an error."] Deny , }
    };
}

Authorization!()