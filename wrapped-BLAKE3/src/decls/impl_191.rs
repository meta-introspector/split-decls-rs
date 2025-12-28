macro_rules! deps {
    () => {
        IncrementCounter!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl IncrementCounter { # [inline] fn yes (& self) -> bool { match self { IncrementCounter :: Yes => true , IncrementCounter :: No => false , } } }
    };
}

impl_191!()