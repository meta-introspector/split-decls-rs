macro_rules! deps {
    () => {
        Error!();
        Regex!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl TryFrom < & str > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
    };
}

impl_84!()