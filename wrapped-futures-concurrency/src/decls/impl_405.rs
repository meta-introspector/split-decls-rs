macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl < S : Stream , const N : usize > ChainTrait for [S ; N] { type Item = S :: Item ; type Stream = Chain < S , N > ; fn chain (self) -> Self :: Stream { Chain { len : self . len () , streams : self , index : 0 , done : false , } } }
    };
}

impl_405!()