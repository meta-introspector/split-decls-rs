macro_rules! deps {
    () => {
        Set!();
        Properties!();
        Color!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Set < Color > for Properties { # [doc = " Sets the line color"] fn set (& mut self , color : Color) -> & mut Properties { self . color = Some (color) ; self } }
    };
}

impl_42!();