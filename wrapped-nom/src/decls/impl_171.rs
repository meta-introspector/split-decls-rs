macro_rules! deps {
    () => {
        OutputM!();
        Mode!();
        IsStreaming!();
        OutputMode!();
        Error!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < M : Mode , EM : Mode , S : IsStreaming > OutputMode for OutputM < M , EM , S > { type Output = M ; type Error = EM ; type Incomplete = S ; }
    };
}

impl_171!();