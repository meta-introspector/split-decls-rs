macro_rules! deps {
    () => {
        Span!();
        LineColLocation!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < Span < '_ > > for LineColLocation { fn from (value : Span < '_ >) -> Self { let (start , end) = value . split () ; Self :: Span (start . line_col () , end . line_col ()) } }
    };
}

impl_9!()