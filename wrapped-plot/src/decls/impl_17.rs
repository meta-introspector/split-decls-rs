macro_rules! deps {
    () => {
        LineType!();
        Display!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Display < & 'static str > for LineType { fn display (& self) -> & 'static str { match * self { LineType :: Dash => "2" , LineType :: Dot => "3" , LineType :: DotDash => "4" , LineType :: DotDotDash => "5" , LineType :: SmallDot => "0" , LineType :: Solid => "1" , } } }
    };
}

impl_17!()