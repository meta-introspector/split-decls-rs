macro_rules! deps {
    () => {
        Result!();
        WorkspaceValue!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl TryFrom < bool > for WorkspaceValue { type Error = String ; fn try_from (other : bool) -> Result < WorkspaceValue , Self :: Error > { if other { Ok (WorkspaceValue) } else { Err ("`workspace` cannot be false" . to_owned ()) } } }
    };
}

impl_107!();