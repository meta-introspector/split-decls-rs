macro_rules! deps {
    () => {
        Display!();
        Order!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Display < & 'static str > for Order { fn display (& self) -> & 'static str { match * self { Order :: TextSample => "noreverse" , Order :: SampleText => "reverse" , } } }
    };
}

impl_18!();