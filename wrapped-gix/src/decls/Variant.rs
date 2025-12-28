macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Variant {
    () => {
        deps!();
        # [doc = " The `ssh.variant` key."] pub type Variant = keys :: Any < validate :: Variant > ;
    };
}

Variant!()