macro_rules! deps {
    () => {
        Display!();
        Horizontal!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Display < & 'static str > for Horizontal { fn display (& self) -> & 'static str { match * self { Horizontal :: Center => "center" , Horizontal :: Left => "left" , Horizontal :: Right => "right" , } } }
    };
}

impl_15!();