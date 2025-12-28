macro_rules! deps {
    () => {
        Properties!();
        Position!();
        Set!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Set < Position > for Properties { # [doc = " Selects where to place the key"] # [doc = ""] # [doc = " **Note** By default, the key is placed `Inside(Vertical::Top, Horizontal::Right)`"] fn set (& mut self , position : Position) -> & mut Properties { self . position = Some (position) ; self } }
    };
}

impl_102!()