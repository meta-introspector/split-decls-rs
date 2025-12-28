macro_rules! deps {
    () => {
        Frame!();
        ResolveWhat!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > ResolveWhat < 'a > { # [allow (dead_code)] fn address_or_ip (& self) -> * mut c_void { match self { ResolveWhat :: Address (a) => adjust_ip (* a) , ResolveWhat :: Frame (f) => adjust_ip (f . ip ()) , } } }
    };
}

impl_14!();