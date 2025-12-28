macro_rules! deps {
    () => {
        Color!();
        Set!();
        Properties!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Set < Color > for Properties { # [doc = " Sets the line color"] fn set (& mut self , color : Color) -> & mut Properties { self . color = Some (color) ; self } }
    };
}

impl_53!();