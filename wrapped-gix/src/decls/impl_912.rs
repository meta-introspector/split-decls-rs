macro_rules! deps {
    () => {
        Name!();
        Url!();
    };
}

macro_rules! impl_912 {
    () => {
        deps!();
        impl Name < '_ > { # [doc = " Obtain the name as string representation."] pub fn as_bstr (& self) -> & BStr { match self { Name :: Symbol (v) => v . as_ref () . into () , Name :: Url (v) => v . as_ref () , } } # [doc = " Return this instance as a symbolic name, if it is one."] pub fn as_symbol (& self) -> Option < & str > { match self { Name :: Symbol (n) => n . as_ref () . into () , Name :: Url (_) => None , } } # [doc = " Return this instance as url, if it is one."] pub fn as_url (& self) -> Option < & BStr > { match self { Name :: Url (n) => n . as_ref () . into () , Name :: Symbol (_) => None , } } # [doc = " Return a fully-owned copy of this instance."] pub fn to_owned (& self) -> Name < 'static > { match self { Name :: Symbol (s) => Name :: Symbol (s . clone () . into_owned () . into ()) , Name :: Url (s) => Name :: Url (s . clone () . into_owned () . into ()) , } } }
    };
}

impl_912!();