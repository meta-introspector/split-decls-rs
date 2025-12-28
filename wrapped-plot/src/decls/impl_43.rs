macro_rules! deps {
    () => {
        Label!();
        Properties!();
        Set!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Set < Label > for Properties { # [doc = " Sets the legend label"] fn set (& mut self , label : Label) -> & mut Properties { self . label = Some (label . 0) ; self } }
    };
}

impl_43!();