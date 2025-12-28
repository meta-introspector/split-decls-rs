macro_rules! deps {
    () => {
        Display!();
        Grid!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Display < & 'static str > for Grid { fn display (& self) -> & 'static str { match * self { Grid :: Major => "" , Grid :: Minor => "m" , } } }
    };
}

impl_14!()