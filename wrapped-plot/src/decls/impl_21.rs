macro_rules! deps {
    () => {
        Display!();
        Terminal!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Display < & 'static str > for Terminal { fn display (& self) -> & 'static str { match * self { Terminal :: Svg => "svg dynamic" , } } }
    };
}

impl_21!();