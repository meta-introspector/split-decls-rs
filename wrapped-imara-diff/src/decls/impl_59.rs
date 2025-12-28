macro_rules! deps {
    () => {
        Score!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Add for Score { type Output = Score ; fn add (self , rhs : Self) -> Self :: Output { Score { indent : self . indent + rhs . indent , penalty : self . penalty + rhs . penalty , } } }
    };
}

impl_59!()