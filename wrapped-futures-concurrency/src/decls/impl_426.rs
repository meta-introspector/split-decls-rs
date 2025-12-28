macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < S : Stream > ChainTrait for Vec < S > { type Item = S :: Item ; type Stream = Chain < S > ; fn chain (self) -> Self :: Stream { Chain { len : self . len () , streams : self , index : 0 , done : false , } } }
    };
}

impl_426!()