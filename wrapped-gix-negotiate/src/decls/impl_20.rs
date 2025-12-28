macro_rules! deps {
    () => {
        Algorithm!();
        Noop!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl std :: fmt :: Display for Algorithm { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Algorithm :: Noop => "noop" , Algorithm :: Consecutive => "consecutive" , Algorithm :: Skipping => "skipping" , } . fmt (f) } }
    };
}

impl_20!();