macro_rules! deps {
    () => {
        Error!();
        Url!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " Deserialization"] impl Url { # [doc = " Parse a URL from `bytes`."] pub fn from_bytes (bytes : & BStr) -> Result < Self , parse :: Error > { parse (bytes) } }
    };
}

impl_56!()