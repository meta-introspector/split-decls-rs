macro_rules! deps {
    () => {
        Display!();
        Style!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Display < & 'static str > for Style { fn display (& self) -> & 'static str { match * self { Style :: XErrorBars => "xerrorbars" , Style :: XErrorLines => "xerrorlines" , Style :: YErrorBars => "yerrorbars" , Style :: YErrorLines => "yerrorlines" , } } }
    };
}

impl_75!();