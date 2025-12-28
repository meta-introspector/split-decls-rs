macro_rules! deps {
    () => {
        Properties!();
        Axes!();
        Set!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Set < Axes > for Properties { # [doc = " Select axes to plot against"] # [doc = ""] # [doc = " **Note** By default, the `BottomXLeftY` axes are used"] fn set (& mut self , axes : Axes) -> & mut Properties { self . axes = Some (axes) ; self } }
    };
}

impl_83!();