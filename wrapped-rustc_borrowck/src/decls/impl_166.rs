macro_rules! deps {
    () => {
        BufferedDiag!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'infcx > BufferedDiag < 'infcx > { fn sort_span (& self) -> Span { match self { BufferedDiag :: Error (diag) => diag . sort_span , BufferedDiag :: NonError (diag) => diag . sort_span , } } }
    };
}

impl_166!();