macro_rules! deps {
    () => {
        Oid!();
        Error!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl str :: FromStr for Oid { type Err = Error ; # [doc = " Parse a hex-formatted object id into an Oid structure."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the string is empty, is longer than 40 hex"] # [doc = " characters, or contains any non-hex characters."] fn from_str (s : & str) -> Result < Oid , Error > { Oid :: from_str (s) } }
    };
}

impl_523!();