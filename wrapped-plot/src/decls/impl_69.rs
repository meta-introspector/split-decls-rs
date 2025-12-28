macro_rules! deps {
    () => {
        Set!();
        Properties!();
        Label!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Set < Label > for Properties { # [doc = " Sets the legend label"] fn set (& mut self , label : Label) -> & mut Properties { self . label = Some (label . 0) ; self } }
    };
}

impl_69!()