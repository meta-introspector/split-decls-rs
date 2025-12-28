macro_rules! deps {
    () => {
        Error!();
        Url!();
        Name!();
    };
}

macro_rules! impl_913 {
    () => {
        deps!();
        impl < 'a > TryFrom < Cow < 'a , BStr > > for Name < 'a > { type Error = Cow < 'a , BStr > ; fn try_from (name : Cow < 'a , BStr >) -> Result < Self , Self :: Error > { if name . contains (& b'/') || name . as_ref () == "." { Ok (Name :: Url (name)) } else { match name { Cow :: Borrowed (n) => n . to_str () . ok () . map (Cow :: Borrowed) . ok_or (name) , Cow :: Owned (n) => Vec :: from (n) . into_string () . map_err (| err | Cow :: Owned (err . into_vec () . into ())) . map (Cow :: Owned) , } . map (Name :: Symbol) } } }
    };
}

impl_913!();