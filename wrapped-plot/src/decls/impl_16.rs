macro_rules! deps {
    () => {
        Display!();
        Justification!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Display < & 'static str > for Justification { fn display (& self) -> & 'static str { match * self { Justification :: Left => "Left" , Justification :: Right => "Right" , } } }
    };
}

impl_16!();