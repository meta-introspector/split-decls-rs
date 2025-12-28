macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : Clone + std :: fmt :: Debug > FileSnapshot < T > { # [doc = " Return the contained instance if nobody else is holding it, or clone it otherwise."] pub fn into_owned_or_cloned (self : OwnShared < Self >) -> T { match OwnShared :: try_unwrap (self) { Ok (this) => this . value , Err (this) => this . value . clone () , } } }
    };
}

impl_10!()