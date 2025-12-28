macro_rules! deps {
    () => {
        Options!();
        Permissions!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl Options { # [doc = " Options configured to prevent accessing anything else than the repository configuration file, prohibiting"] # [doc = " accessing the environment or spreading beyond the git repository location."] pub fn isolated () -> Self { Options :: default () . permissions (Permissions :: isolated ()) } }
    };
}

impl_498!()