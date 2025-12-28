macro_rules! deps {
    () => {
        Algorithm!();
        Myers!();
        Histogram!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Algorithm { # [cfg (test)] const ALL : [Self ; 2] = [Algorithm :: Histogram , Algorithm :: Myers] ; }
    };
}

impl_108!()