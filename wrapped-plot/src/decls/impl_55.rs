macro_rules! deps {
    () => {
        Properties!();
        Set!();
        LineType!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Set < LineType > for Properties { # [doc = " Changes the line type"] # [doc = ""] # [doc = " **Note** By default `Solid` lines are used"] fn set (& mut self , lt : LineType) -> & mut Properties { self . line_type = lt ; self } }
    };
}

impl_55!();