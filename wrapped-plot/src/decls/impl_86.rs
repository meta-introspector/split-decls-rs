macro_rules! deps {
    () => {
        Opacity!();
        Set!();
        Properties!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Set < Opacity > for Properties { # [doc = " Changes the opacity of the fill color"] # [doc = ""] # [doc = " **Note** By default, the fill color is totally opaque (`opacity = 1.0`)"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `opacity` is outside the range `[0, 1]`"] fn set (& mut self , opacity : Opacity) -> & mut Properties { self . opacity = Some (opacity . 0) ; self } }
    };
}

impl_86!()