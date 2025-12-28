macro_rules! deps {
    () => {
        Credential!();
    };
}

macro_rules! UnsupportedCredential {
    () => {
        deps!();
        # [doc = " Credential provider that doesn't support any registries."] pub struct UnsupportedCredential ;
    };
}

UnsupportedCredential!();