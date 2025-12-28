macro_rules! deps {
    () => {
        Type!();
        Reader!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl std :: ops :: Deref for Reader { type Target = HashMap < & 'static str , HashMap < & 'static str , Vec < Type > > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_456!();