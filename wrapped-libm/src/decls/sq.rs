macro_rules! sq {
    () => {
        fn sq (x : f64) -> (f64 , f64) { let xh : f64 ; let xl : f64 ; let xc : f64 ; xc = x * SPLIT ; xh = x - xc + xc ; xl = x - xh ; let hi = x * x ; let lo = xh * xh - hi + 2. * xh * xl + xl * xl ; (hi , lo) }
    };
}

sq!();