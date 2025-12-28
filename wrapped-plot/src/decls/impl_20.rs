macro_rules! deps {
    () => {
        Stacked!();
        Display!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Display < & 'static str > for Stacked { fn display (& self) -> & 'static str { match * self { Stacked :: Horizontally => "horizontal" , Stacked :: Vertically => "vertical" , } } }
    };
}

impl_20!()