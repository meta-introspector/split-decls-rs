macro_rules! deps {
    () => {
        SessionKeys!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl SessionKeys { # [doc = " Get the shared secret intended to be used for receiving data from the other party."] pub fn receiving (& self) -> & SecretKey { & self . rx } # [doc = " Get the shared secret intended to be used for transporting data to the other party."] pub fn transport (& self) -> & SecretKey { & self . tx } }
    };
}

impl_628!();