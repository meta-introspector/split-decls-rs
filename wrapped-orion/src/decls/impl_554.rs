macro_rules! deps {
    () => {
        DecapsulationKey!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl DecapsulationKey { # [inline] # [doc = " Return the object as byte slice. __**Warning**__: Should not be used unless strictly"] # [doc = " needed. This __**breaks protections**__ that the type implements."] pub fn unprotected_as_bytes (& self) -> & [u8] { self . seed . unprotected_as_bytes () } }
    };
}

impl_554!()