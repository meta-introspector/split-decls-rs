macro_rules! deps {
    () => {
        RawAttrs!();
        Attr!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl ops :: Deref for RawAttrs { type Target = [Attr] ; fn deref (& self) -> & [Attr] { match & self . entries { Some (it) => & it . slice , None => & [] , } } }
    };
}

impl_1!()