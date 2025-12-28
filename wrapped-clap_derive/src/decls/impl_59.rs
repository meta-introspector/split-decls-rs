macro_rules! deps {
    () => {
        Action!();
        Method!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Action { pub (crate) fn resolve (self , _field_type : & Type) -> Method { match self { Self :: Explicit (method) => method , Self :: Implicit (ident) => default_action (_field_type , ident . span ()) , } } pub (crate) fn span (& self) -> Span { match self { Self :: Explicit (method) => method . name . span () , Self :: Implicit (ident) => ident . span () , } } }
    };
}

impl_59!()