macro_rules! deps {
    () => {
        Properties!();
        CurveDefault!();
        Style!();
        LineType!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl CurveDefault < Style > for Properties { fn default (style : Style) -> Properties { Properties { axes : None , color : None , label : None , line_type : LineType :: Solid , linewidth : None , point_size : None , point_type : None , style , } } }
    };
}

impl_50!()