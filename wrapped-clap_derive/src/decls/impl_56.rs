macro_rules! deps {
    () => {
        ValueParser!();
        Method!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl ValueParser { fn resolve (self , _inner_type : & Type) -> Method { match self { Self :: Explicit (method) => method , Self :: Implicit (ident) => default_value_parser (_inner_type , ident . span ()) , } } fn span (& self) -> Span { match self { Self :: Explicit (method) => method . name . span () , Self :: Implicit (ident) => ident . span () , } } }
    };
}

impl_56!()