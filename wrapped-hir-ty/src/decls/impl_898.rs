macro_rules! deps {
    () => {
        StatementKind!();
        MirSpan!();
        Statement!();
    };
}

macro_rules! impl_898 {
    () => {
        deps!();
        impl < 'db > StatementKind < 'db > { fn with_span (self , span : MirSpan) -> Statement < 'db > { Statement { kind : self , span } } }
    };
}

impl_898!();