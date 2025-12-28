macro_rules! deps {
    () => {
        Estimates!();
        Statistic!();
        Slope!();
        Estimate!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Estimates { pub fn typical (& self) -> & Estimate { self . slope . as_ref () . unwrap_or (& self . mean) } pub fn get (& self , stat : Statistic) -> Option < & Estimate > { match stat { Statistic :: Mean => Some (& self . mean) , Statistic :: Median => Some (& self . median) , Statistic :: MedianAbsDev => Some (& self . median_abs_dev) , Statistic :: Slope => self . slope . as_ref () , Statistic :: StdDev => Some (& self . std_dev) , Statistic :: Typical => Some (self . typical ()) , } } }
    };
}

impl_101!();