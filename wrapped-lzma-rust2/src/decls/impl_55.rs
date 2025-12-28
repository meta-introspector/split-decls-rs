macro_rules! deps {
    () => {
        LengthCoder!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl LengthCoder { pub fn new () -> Self { Self { choice : Default :: default () , low : Default :: default () , mid : Default :: default () , high : [0 ; HIGH_SYMBOLS] , } } pub fn reset (& mut self) { init_probs (& mut self . choice) ; for ele in self . low . iter_mut () { init_probs (ele) ; } for ele in self . mid . iter_mut () { init_probs (ele) ; } init_probs (& mut self . high) ; } }
    };
}

impl_55!()