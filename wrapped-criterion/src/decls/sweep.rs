macro_rules! deps {
    () => {
        Sample!();
    };
}

macro_rules! sweep {
    () => {
        deps!();
        pub fn sweep (sample : & Sample < f64 > , npoints : usize , range : Option < (f64 , f64) > ,) -> (Box < [f64] > , Box < [f64] >) { let (xs , ys , _) = sweep_and_estimate (sample , npoints , range , sample [0]) ; (xs , ys) }
    };
}

sweep!();