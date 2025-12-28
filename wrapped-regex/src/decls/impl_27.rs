macro_rules! deps {
    () => {
        Regex!();
        Error!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl TryFrom < String > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : String) -> Result < Regex , Error > { Regex :: new (& s) } }
    };
}

impl_27!();