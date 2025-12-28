macro_rules! deps {
    () => {
        Estimate!();
        Statistic!();
        ChangeEstimates!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl ChangeEstimates { pub fn get (& self , stat : Statistic) -> & Estimate { match stat { Statistic :: Mean => & self . mean , Statistic :: Median => & self . median , _ => panic ! ("Unexpected statistic") , } } }
    };
}

impl_106!();