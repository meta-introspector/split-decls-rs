macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < T : Into < PollTimeout > > From < Option < T > > for PollTimeout { fn from (x : Option < T >) -> Self { x . map_or (Self :: NONE , | x | x . into ()) } }
    };
}

impl_248!()