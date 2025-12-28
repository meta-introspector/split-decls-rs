macro_rules! deps {
    () => {
        Algorithm!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Algorithm { # [cfg (test)] const ALL : [Self ; 2] = [Algorithm :: Histogram , Algorithm :: Myers] ; }
    };
}

impl_10!()