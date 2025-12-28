macro_rules! deps {
    () => {
        TrustedToken!();
        Error!();
        Footer!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl TryFrom < & TrustedToken > for Footer { type Error = Error ; fn try_from (value : & TrustedToken) -> Result < Self , Self :: Error > { if value . footer . is_empty () { return Err (Error :: FooterParsing) ; } let mut footer = Footer :: new () ; footer . parse_bytes (value . footer ()) ? ; Ok (footer) } }
    };
}

impl_128!()