macro_rules! deps {
    () => {
        StringSink!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [cfg (any (feature = "alloc" , test))] impl < 'a > StringSink < 'a > { pub (crate) fn new (s : & mut String) -> StringSink < '_ > { StringSink { string : s } } }
    };
}

impl_5!()