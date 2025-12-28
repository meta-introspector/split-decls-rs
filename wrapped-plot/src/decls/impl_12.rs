macro_rules! deps {
    () => {
        Display!();
        Axes!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Display < & 'static str > for Axes { fn display (& self) -> & 'static str { match * self { Axes :: BottomXLeftY => "x1y1" , Axes :: BottomXRightY => "x1y2" , Axes :: TopXLeftY => "x2y1" , Axes :: TopXRightY => "x2y2" , } } }
    };
}

impl_12!()