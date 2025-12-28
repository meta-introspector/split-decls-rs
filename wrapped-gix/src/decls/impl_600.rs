macro_rules! deps {
    () => {
        Clone!();
        Boolean!();
        Tree!();
        RemoteName!();
    };
}

macro_rules! impl_600 {
    () => {
        deps!();
        impl Clone { # [doc = " The `clone.defaultRemoteName` key."] pub const DEFAULT_REMOTE_NAME : keys :: RemoteName = keys :: RemoteName :: new_remote_name ("defaultRemoteName" , & config :: Tree :: CLONE) ; # [doc = " The `clone.rejectShallow` key."] pub const REJECT_SHALLOW : keys :: Boolean = keys :: Boolean :: new_boolean ("rejectShallow" , & config :: Tree :: CLONE) ; }
    };
}

impl_600!();