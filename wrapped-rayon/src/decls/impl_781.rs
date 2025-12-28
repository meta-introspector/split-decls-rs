macro_rules! deps {
    () => {
        Reducer!();
        ProductConsumer!();
    };
}

macro_rules! impl_781 {
    () => {
        deps!();
        impl < P > Reducer < P > for ProductConsumer < P > where P : Send + Product , { fn reduce (self , left : P , right : P) -> P { mul (left , right) } }
    };
}

impl_781!();