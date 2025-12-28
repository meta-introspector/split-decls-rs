macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! SymmetricDifference {
    () => {
        deps!();
        pub struct SymmetricDifference < 'a , T , S > { iter : Chain < Difference < 'a , T , S > , Difference < 'a , T , S > > , }
    };
}

SymmetricDifference!();