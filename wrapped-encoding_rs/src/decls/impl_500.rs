macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        # [cfg (test)] impl PartialOrd for Encoding { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { (self as * const Encoding as usize) . partial_cmp (& (other as * const Encoding as usize)) } }
    };
}

impl_500!();