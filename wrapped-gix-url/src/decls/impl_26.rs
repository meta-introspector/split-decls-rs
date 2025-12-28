macro_rules! deps {
    () => {
        UrlKind!();
        Url!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl UrlKind { fn as_str (& self) -> & 'static str { match self { UrlKind :: Url => "URL" , UrlKind :: Scp => "SCP-like target" , UrlKind :: Local => "local path" , } } }
    };
}

impl_26!()