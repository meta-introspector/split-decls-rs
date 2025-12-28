macro_rules! deps {
    () => {
        Properties!();
        ErrorBarDefault!();
        LineType!();
        Style!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ErrorBarDefault < Style > for Properties { fn default (style : Style) -> Properties { Properties { color : None , label : None , line_type : LineType :: Solid , linewidth : None , point_type : None , point_size : None , style , } } }
    };
}

impl_66!()