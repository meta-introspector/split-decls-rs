macro_rules! deps {
    () => {
        Properties!();
        Scale!();
        Set!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Set < Scale > for Properties { # [doc = " Sets the scale of the axis"] # [doc = ""] # [doc = " **Note** All axes use a linear scale by default"] fn set (& mut self , scale : Scale) -> & mut Properties { self . hidden = false ; match scale { Scale :: Linear => self . logarithmic = false , Scale :: Logarithmic => self . logarithmic = true , } self } }
    };
}

impl_33!();