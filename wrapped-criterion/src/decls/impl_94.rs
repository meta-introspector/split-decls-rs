macro_rules! deps {
    () => {
        Slope!();
        Result!();
        Statistic!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl fmt :: Display for Statistic { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Statistic :: Mean => f . pad ("mean") , Statistic :: Median => f . pad ("median") , Statistic :: MedianAbsDev => f . pad ("MAD") , Statistic :: Slope => f . pad ("slope") , Statistic :: StdDev => f . pad ("SD") , Statistic :: Typical => f . pad ("typical") , } } }
    };
}

impl_94!();