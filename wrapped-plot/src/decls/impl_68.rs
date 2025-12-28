macro_rules! deps {
    () => {
        Set!();
        Color!();
        Properties!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl Set < Color > for Properties { # [doc = " Changes the color of the error bars"] fn set (& mut self , color : Color) -> & mut Properties { self . color = Some (color) ; self } }
    };
}

impl_68!()