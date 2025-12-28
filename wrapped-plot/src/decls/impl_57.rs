macro_rules! deps {
    () => {
        Set!();
        Properties!();
        PointSize!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Set < PointSize > for Properties { # [doc = " Changes the size of the points"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `size` is a non-positive value"] fn set (& mut self , ps : PointSize) -> & mut Properties { let ps = ps . 0 ; assert ! (ps > 0.) ; self . point_size = Some (ps) ; self } }
    };
}

impl_57!()