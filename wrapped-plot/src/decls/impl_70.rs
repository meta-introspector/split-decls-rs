macro_rules! deps {
    () => {
        Properties!();
        LineType!();
        Set!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Set < LineType > for Properties { # [doc = " Change the line type"] # [doc = ""] # [doc = " **Note** By default `Solid` lines are used"] fn set (& mut self , lt : LineType) -> & mut Properties { self . line_type = lt ; self } }
    };
}

impl_70!()