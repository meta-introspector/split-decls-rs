macro_rules! deps {
    () => {
        MacroKinds!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl MacroKinds { # [doc = " Convert the MacroKinds to a static string."] # [doc = ""] # [doc = " This hardcodes all the possibilities, in order to return a static string."] pub fn descr (self) -> & 'static str { match self { Self :: BANG => "macro" , Self :: ATTR => "attribute macro" , Self :: DERIVE => "derive macro" , _ if self == (Self :: ATTR | Self :: BANG) => "attribute/function macro" , _ if self == (Self :: DERIVE | Self :: BANG) => "derive/function macro" , _ if self == (Self :: ATTR | Self :: DERIVE) => "attribute/derive macro" , _ if self . is_all () => "attribute/derive/function macro" , _ if self . is_empty () => "useless macro" , _ => unreachable ! () , } } # [doc = " Return an indefinite article (a/an) for use with `descr()`"] pub fn article (self) -> & 'static str { if self . contains (Self :: ATTR) { "an" } else { "a" } } }
    };
}

impl_56!()