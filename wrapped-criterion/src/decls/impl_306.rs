macro_rules! deps {
    () => {
        Data!();
        Pairs!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < 'a , X , Y > Data < 'a , X , Y > { # [doc = " Returns the length of the data set"] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Iterate over the data set"] pub fn iter (& self) -> Pairs < 'a , X , Y > { Pairs { data : * self , state : 0 , } } }
    };
}

impl_306!();