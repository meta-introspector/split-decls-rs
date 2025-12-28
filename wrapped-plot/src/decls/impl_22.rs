macro_rules! deps {
    () => {
        Terminal!();
        Figure!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Set < Terminal > for Figure { # [doc = " Changes the output terminal"] # [doc = ""] # [doc = " **Note** By default, the terminal is set to `Svg`"] fn set (& mut self , terminal : Terminal) -> & mut Figure { self . terminal = terminal ; self } }
    };
}

impl_22!()