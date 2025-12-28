macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > Sp < T > { pub (crate) fn new (val : T , span : Span) -> Self { Sp { val , span } } pub (crate) fn get (& self) -> & T { & self . val } pub (crate) fn span (& self) -> Span { self . span } }
    };
}

impl_92!()