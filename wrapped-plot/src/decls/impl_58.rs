macro_rules! deps {
    () => {
        Set!();
        PointType!();
        Properties!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Set < PointType > for Properties { # [doc = " Changes the point type"] fn set (& mut self , pt : PointType) -> & mut Properties { self . point_type = Some (pt) ; self } }
    };
}

impl_58!();