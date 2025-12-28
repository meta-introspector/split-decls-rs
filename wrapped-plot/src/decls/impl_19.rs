macro_rules! deps {
    () => {
        Display!();
        PointType!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Display < & 'static str > for PointType { fn display (& self) -> & 'static str { match * self { PointType :: Circle => "6" , PointType :: FilledCircle => "7" , PointType :: FilledSquare => "5" , PointType :: FilledTriangle => "9" , PointType :: Plus => "1" , PointType :: Square => "4" , PointType :: Star => "3" , PointType :: Triangle => "8" , PointType :: X => "2" , } } }
    };
}

impl_19!();