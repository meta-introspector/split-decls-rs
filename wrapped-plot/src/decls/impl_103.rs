macro_rules! deps {
    () => {
        Set!();
        Stacked!();
        Properties!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Set < Stacked > for Properties { # [doc = " Changes how the entries of the key are stacked"] fn set (& mut self , stacked : Stacked) -> & mut Properties { self . stacked = Some (stacked) ; self } }
    };
}

impl_103!()