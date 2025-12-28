macro_rules! deps {
    () => {
        Action!();
        Error!();
        Result!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TryFrom < OsString > for Action { type Error = Error ; fn try_from (value : OsString) -> Result < Self , Self :: Error > { Ok (match value . to_str () { Some ("fill" | "get") => Action :: Get , Some ("approve" | "store") => Action :: Store , Some ("reject" | "erase") => Action :: Erase , _ => return Err (Error :: ActionInvalid { name : value }) , }) } }
    };
}

impl_28!();