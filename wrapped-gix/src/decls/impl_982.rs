macro_rules! deps {
    () => {
        Allow!();
        Error!();
        User!();
    };
}

macro_rules! impl_982 {
    () => {
        deps!();
        impl < 'a > TryFrom < Cow < 'a , BStr > > for Allow { type Error = BString ; fn try_from (v : Cow < 'a , BStr >) -> Result < Self , Self :: Error > { Ok (match v . as_ref () . as_bytes () { b"never" => Allow :: Never , b"always" => Allow :: Always , b"user" => Allow :: User , unknown => return Err (unknown . into ()) , }) } }
    };
}

impl_982!();