macro_rules! deps {
    () => {
        Display!();
        Style!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Display < & 'static str > for Style { fn display (& self) -> & 'static str { match * self { Style :: Dots => "dots" , Style :: Impulses => "impulses" , Style :: Lines => "lines" , Style :: LinesPoints => "linespoints" , Style :: Points => "points" , Style :: Steps => "steps" , } } }
    };
}

impl_62!();