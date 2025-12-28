macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Display for Edition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Edition :: Edition2015 => "2015" , Edition :: Edition2018 => "2018" , Edition :: Edition2021 => "2021" , Edition :: Edition2024 => "2024" , }) } }
    };
}

impl_6!();