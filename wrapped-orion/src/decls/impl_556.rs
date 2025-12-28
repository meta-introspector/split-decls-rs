macro_rules! deps {
    () => {
        UnknownCryptoError!();
        KeyPair!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { KeyPair :: generate_deterministic (value) } }
    };
}

impl_556!();