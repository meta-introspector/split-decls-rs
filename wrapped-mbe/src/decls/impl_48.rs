macro_rules! deps {
    () => {
        ExpandErrorKind!();
        ExpandError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl ExpandError { fn new (span : Span , kind : ExpandErrorKind) -> ExpandError { ExpandError { inner : Arc :: new ((span , kind)) } } fn binding_error (span : Span , e : impl Into < Box < str > >) -> ExpandError { ExpandError { inner : Arc :: new ((span , ExpandErrorKind :: BindingError (Box :: new (e . into ())))) } } }
    };
}

impl_48!()