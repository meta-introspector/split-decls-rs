macro_rules! deps {
    () => {
        Set!();
        LineType!();
        Properties!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Set < LineType > for Properties { # [doc = " Changes the line type"] # [doc = ""] # [doc = " **Note** By default `Solid` lines are used"] fn set (& mut self , lt : LineType) -> & mut Properties { self . line_type = lt ; self } }
    };
}

impl_44!()