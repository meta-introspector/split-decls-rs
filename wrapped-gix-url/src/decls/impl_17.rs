macro_rules! deps {
    () => {
        Url!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " Deserialization"] impl Url { # [doc = " Parse a URL from `bytes`."] pub fn from_bytes (bytes : & BStr) -> Result < Self , parse :: Error > { parse (bytes) } }
    };
}

impl_17!()