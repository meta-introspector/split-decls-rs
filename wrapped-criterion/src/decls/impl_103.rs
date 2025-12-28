macro_rules! deps {
    () => {
        Statistic!();
        Slope!();
        Distribution!();
        Distributions!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Distributions { pub fn typical (& self) -> & Distribution < f64 > { self . slope . as_ref () . unwrap_or (& self . mean) } pub fn get (& self , stat : Statistic) -> Option < & Distribution < f64 > > { match stat { Statistic :: Mean => Some (& self . mean) , Statistic :: Median => Some (& self . median) , Statistic :: MedianAbsDev => Some (& self . median_abs_dev) , Statistic :: Slope => self . slope . as_ref () , Statistic :: StdDev => Some (& self . std_dev) , Statistic :: Typical => Some (self . typical ()) , } } }
    };
}

impl_103!()