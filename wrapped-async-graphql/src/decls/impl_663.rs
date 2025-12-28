macro_rules! deps {
    () => {
        ClientMessage!();
        Result!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl ClientMessage { # [doc = " Creates a ClientMessage from an array of bytes"] pub fn from_bytes < T > (message : T) -> serde_json :: Result < Self > where T : AsRef < [u8] > , { serde_json :: from_slice (message . as_ref ()) } }
    };
}

impl_663!();