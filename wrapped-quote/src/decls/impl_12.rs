macro_rules! deps {
    () => {
        IdentFragment!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl IdentFragment for Ident { fn span (& self) -> Option < Span > { Some (self . span ()) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let id = self . to_string () ; if let Some (id) = id . strip_prefix ("r#") { fmt :: Display :: fmt (id , f) } else { fmt :: Display :: fmt (& id [..] , f) } } }
    };
}

impl_12!();