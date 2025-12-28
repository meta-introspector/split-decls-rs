macro_rules! deps {
    () => {
        Display!();
        Axis!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Display < & 'static str > for Axis { fn display (& self) -> & 'static str { match * self { Axis :: BottomX => "x" , Axis :: LeftY => "y" , Axis :: RightY => "y2" , Axis :: TopX => "x2" , } } }
    };
}

impl_11!();