macro_rules! deps {
    () => {
        MultiSpan!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < Vec < Span > > for MultiSpan { fn from (spans : Vec < Span >) -> MultiSpan { MultiSpan :: from_spans (spans) } }
    };
}

impl_27!()