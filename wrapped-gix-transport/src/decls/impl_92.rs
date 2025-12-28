macro_rules! deps {
    () => {
        Transport!();
        Http!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < H : Http > Transport < H > { # [doc = " Returns the identity that the transport uses when connecting to the remote."] pub fn identity (& self) -> Option < & gix_sec :: identity :: Account > { self . identity . as_ref () } }
    };
}

impl_92!()