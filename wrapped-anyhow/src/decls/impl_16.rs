macro_rules! deps {
    () => {
        StdError!();
        ChainState!();
        Chain!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > Chain < 'a > { # [cold] pub fn new (head : & 'a (dyn StdError + 'static)) -> Self { Chain { state : ChainState :: Linked { next : Some (head) } , } } }
    };
}

impl_16!()