macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        # [cfg (test)] impl Ord for Encoding { fn cmp (& self , other : & Self) -> Ordering { (self as * const Encoding as usize) . cmp (& (other as * const Encoding as usize)) } }
    };
}

impl_501!();