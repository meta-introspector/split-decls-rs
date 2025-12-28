macro_rules! deps {
    () => {
        Steal!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > FromIterator < Self > for Steal < T > { # [doc = " Consumes items until a `Success` is found and returns it."] # [doc = ""] # [doc = " If no `Success` was found, but there was at least one `Retry`, then returns `Retry`."] # [doc = " Otherwise, `Empty` is returned."] fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Self > , { let mut retry = false ; for s in iter { match & s { Self :: Empty => { } Self :: Success (_) => return s , Self :: Retry => retry = true , } } if retry { Self :: Retry } else { Self :: Empty } } }
    };
}

impl_44!()