macro_rules! deps {
    () => {
        Set!();
        Label!();
        Properties!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Set < Label > for Properties { # [doc = " Sets the legend label"] fn set (& mut self , label : Label) -> & mut Properties { self . label = Some (label . 0) ; self } }
    };
}

impl_54!();