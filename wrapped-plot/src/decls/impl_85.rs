macro_rules! deps {
    () => {
        Properties!();
        Label!();
        Set!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Set < Label > for Properties { # [doc = " Sets the legend label"] fn set (& mut self , label : Label) -> & mut Properties { self . label = Some (label . 0) ; self } }
    };
}

impl_85!();