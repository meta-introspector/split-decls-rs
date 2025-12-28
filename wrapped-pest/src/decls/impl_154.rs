macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < 'i > PartialEq for Span < 'i > { fn eq (& self , other : & Span < 'i >) -> bool { ptr :: eq (self . input , other . input) && self . start == other . start && self . end == other . end } }
    };
}

impl_154!();