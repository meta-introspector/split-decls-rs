macro_rules! deps {
    () => {
        ChangeDistributions!();
        Statistic!();
        Distribution!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl ChangeDistributions { pub fn get (& self , stat : Statistic) -> & Distribution < f64 > { match stat { Statistic :: Mean => & self . mean , Statistic :: Median => & self . median , _ => panic ! ("Unexpected statistic") , } } }
    };
}

impl_108!();