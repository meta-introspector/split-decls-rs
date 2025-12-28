macro_rules! deps {
    () => {
        Order!();
        Set!();
        Properties!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Set < Order > for Properties { # [doc = " How to order each entry"] # [doc = ""] # [doc = " **Note** The default order is `TextSample`"] fn set (& mut self , order : Order) -> & mut Properties { self . order = Some (order) ; self } }
    };
}

impl_101!()