macro_rules! deps {
    () => {
        Allow!();
        Protocol!();
        NameParameter!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl NameParameter { # [doc = " The `protocol.<name>.allow` key."] pub const ALLOW : Allow = Allow :: new_with_validate ("allow" , & Protocol :: NAME_PARAMETER , validate :: Allow) ; }
    };
}

impl_707!();