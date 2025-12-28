macro_rules! deps {
    () => {
        Variant!();
        Tree!();
        Ssh!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl Ssh { # [doc = " The `ssh.variant` key"] pub const VARIANT : Variant = Variant :: new_with_validate ("variant" , & config :: Tree :: SSH , validate :: Variant) . with_environment_override ("GIT_SSH_VARIANT") . with_deviation ("We error if a variant is chosen that we don't know, as opposed to defaulting to 'ssh'") ; }
    };
}

impl_734!();