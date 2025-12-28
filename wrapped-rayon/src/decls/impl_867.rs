macro_rules! deps {
    () => {
        SumConsumer!();
        Reducer!();
    };
}

macro_rules! impl_867 {
    () => {
        deps!();
        impl < S > Reducer < S > for SumConsumer < S > where S : Send + Sum , { fn reduce (self , left : S , right : S) -> S { add (left , right) } }
    };
}

impl_867!();