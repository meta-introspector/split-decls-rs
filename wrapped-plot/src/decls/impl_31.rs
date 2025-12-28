macro_rules! deps {
    () => {
        Properties!();
        Set!();
        Label!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Set < Label > for Properties { # [doc = " Attaches a label to the axis"] fn set (& mut self , label : Label) -> & mut Properties { self . label = Some (label . 0) ; self } }
    };
}

impl_31!();