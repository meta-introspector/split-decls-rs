macro_rules! deps {
    () => {
        Axes!();
        Properties!();
        Set!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Set < Axes > for Properties { # [doc = " Select the axes to plot against"] # [doc = ""] # [doc = " **Note** By default, the `BottomXLeftY` axes are used"] fn set (& mut self , axes : Axes) -> & mut Properties { self . axes = Some (axes) ; self } }
    };
}

impl_52!()