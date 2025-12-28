macro_rules! deps {
    () => {
        DiagCtxtHandle!();
        DiagCtxt!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > std :: ops :: Deref for DiagCtxtHandle < 'a > { type Target = & 'a DiagCtxt ; fn deref (& self) -> & Self :: Target { & self . dcx } }
    };
}

impl_38!()