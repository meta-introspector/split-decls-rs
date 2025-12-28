macro_rules! deps {
    () => {
        Display!();
        Vertical!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Display < & 'static str > for Vertical { fn display (& self) -> & 'static str { match * self { Vertical :: Bottom => "bottom" , Vertical :: Center => "center" , Vertical :: Top => "top" , } } }
    };
}

impl_22!();