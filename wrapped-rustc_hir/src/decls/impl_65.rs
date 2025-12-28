macro_rules! deps {
    () => {
        Namespace!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Namespace { # [doc = " The English description of the namespace."] pub fn descr (self) -> & 'static str { match self { Self :: TypeNS => "type" , Self :: ValueNS => "value" , Self :: MacroNS => "macro" , } } }
    };
}

impl_65!();