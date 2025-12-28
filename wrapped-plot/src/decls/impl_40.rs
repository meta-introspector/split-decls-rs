macro_rules! deps {
    () => {
        Properties!();
        Default!();
        LineType!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Default for Properties { fn default () -> Properties { Properties { color : None , label : None , line_type : LineType :: Solid , linewidth : None , } } }
    };
}

impl_40!();